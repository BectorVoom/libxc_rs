//! Mathematical constants from libxc `util.h`, truncated to f64 (17 significant digits).

/// sqrt(pi) = 1.7724538509055160
pub const M_SQRTPI: f64 = 1.7724538509055160;

/// cbrt(pi) = 1.4645918875615232
pub const M_CBRTPI: f64 = 1.4645918875615232;

/// sqrt(3) = 1.7320508075688773
pub const M_SQRT3: f64 = 1.7320508075688773;

/// cbrt(2) = 1.2599210498948732
pub const M_CBRT2: f64 = 1.2599210498948732;

/// cbrt(3) = 1.4422495703074084
pub const M_CBRT3: f64 = 1.4422495703074084;

/// cbrt(4) = 1.5874010519681994
pub const M_CBRT4: f64 = 1.5874010519681994;

/// cbrt(5) = 1.7099759466766970
pub const M_CBRT5: f64 = 1.7099759466766970;

/// cbrt(6) = 1.8171205928321397
pub const M_CBRT6: f64 = 1.8171205928321397;

/// cbrt(7) = 1.9129311827723891
pub const M_CBRT7: f64 = 1.9129311827723891;

/// cbrt(9) = 2.0800838230519041
pub const M_CBRT9: f64 = 2.0800838230519041;

/// sqrt(2) -- use Rust standard library value
pub const M_SQRT2: f64 = std::f64::consts::SQRT_2;

/// Speed of light in atomic units
pub const M_C: f64 = 137.0359996287515;

/// (3/(4*pi))^(1/3) -- Wigner-Seitz radius prefactor
pub const RS_FACTOR: f64 = 0.62035049089940002;

/// 3/8 * cbrt(3/pi) * 4^(2/3) -- Exchange factor
pub const X_FACTOR_C: f64 = 0.93052573634910003;

/// 8/(3*sqrt(pi)) -- 2D exchange factor
pub const X_FACTOR_2D_C: f64 = 1.5045055561273501;

/// 3/10*(6*pi^2)^(2/3) -- Thomas-Fermi kinetic energy prefactor
pub const K_FACTOR_C: f64 = 4.5577998723455971;

/// 10/81 -- GE2 gradient expansion coefficient
pub const MU_GE: f64 = 0.12345679012345679;

/// mu_PBE = beta*pi^2/3
pub const MU_PBE: f64 = 0.2195149727645171;

/// 1/(2*(6*pi^2)^(1/3)) -- gradient to reduced gradient conversion
pub const X2S: f64 = 0.12827824385304219;

/// 1/(2*(4*pi)^(1/2)) -- 2D gradient conversion
pub const X2S_2D: f64 = 0.14104739588693907;

/// 2^(4/3) - 2 -- spin scaling factor
pub const FZETAFACTOR: f64 = 0.51984209978974633;

/// (3*pi^2)^(1/3) -- Fermi wavevector prefactor
pub const KF_CONST: f64 = 3.0936677262801355;

/// Alias for RS_FACTOR for clarity in DFT quantity functions
pub const RS_CONST: f64 = RS_FACTOR;

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: compute relative error between two f64 values
    fn rel_err(a: f64, b: f64) -> f64 {
        if b == 0.0 {
            a.abs()
        } else {
            ((a - b) / b).abs()
        }
    }

    #[test]
    fn test_sqrt_constants() {
        // M_SQRT2 matches Rust std
        assert_eq!(M_SQRT2, std::f64::consts::SQRT_2);

        // M_SQRT3 matches computed value
        let computed_sqrt3 = 3.0_f64.sqrt();
        assert!(rel_err(M_SQRT3, computed_sqrt3) < 1e-15,
            "M_SQRT3 mismatch: {} vs {}", M_SQRT3, computed_sqrt3);

        // M_SQRTPI matches computed value
        let computed_sqrtpi = std::f64::consts::PI.sqrt();
        assert!(rel_err(M_SQRTPI, computed_sqrtpi) < 1e-15,
            "M_SQRTPI mismatch: {} vs {}", M_SQRTPI, computed_sqrtpi);
    }

    #[test]
    fn test_cbrt_constants() {
        let computed_cbrt2 = 2.0_f64.cbrt();
        assert!(rel_err(M_CBRT2, computed_cbrt2) < 1e-15,
            "M_CBRT2 mismatch: {} vs {}", M_CBRT2, computed_cbrt2);

        let computed_cbrt3 = 3.0_f64.cbrt();
        assert!(rel_err(M_CBRT3, computed_cbrt3) < 1e-15,
            "M_CBRT3 mismatch: {} vs {}", M_CBRT3, computed_cbrt3);

        let computed_cbrt4 = 4.0_f64.cbrt();
        assert!(rel_err(M_CBRT4, computed_cbrt4) < 1e-15,
            "M_CBRT4 mismatch: {} vs {}", M_CBRT4, computed_cbrt4);

        let computed_cbrt5 = 5.0_f64.cbrt();
        assert!(rel_err(M_CBRT5, computed_cbrt5) < 1e-15,
            "M_CBRT5 mismatch: {} vs {}", M_CBRT5, computed_cbrt5);

        let computed_cbrt6 = 6.0_f64.cbrt();
        assert!(rel_err(M_CBRT6, computed_cbrt6) < 1e-15,
            "M_CBRT6 mismatch: {} vs {}", M_CBRT6, computed_cbrt6);

        let computed_cbrt7 = 7.0_f64.cbrt();
        assert!(rel_err(M_CBRT7, computed_cbrt7) < 1e-15,
            "M_CBRT7 mismatch: {} vs {}", M_CBRT7, computed_cbrt7);

        let computed_cbrt9 = 9.0_f64.cbrt();
        assert!(rel_err(M_CBRT9, computed_cbrt9) < 1e-15,
            "M_CBRT9 mismatch: {} vs {}", M_CBRT9, computed_cbrt9);

        let computed_cbrtpi = std::f64::consts::PI.cbrt();
        assert!(rel_err(M_CBRTPI, computed_cbrtpi) < 1e-15,
            "M_CBRTPI mismatch: {} vs {}", M_CBRTPI, computed_cbrtpi);
    }

    #[test]
    fn test_derived_constants() {
        // RS_FACTOR = (3/(4*pi))^(1/3)
        let computed_rs = (3.0 / (4.0 * std::f64::consts::PI)).cbrt();
        assert!(rel_err(RS_FACTOR, computed_rs) < 1e-15,
            "RS_FACTOR mismatch: {} vs {}", RS_FACTOR, computed_rs);

        // X_FACTOR_C = 3/8 * cbrt(3/pi) * 4^(2/3)
        let computed_xfc = 0.375 * (3.0 / std::f64::consts::PI).cbrt() * 4.0_f64.powf(2.0 / 3.0);
        assert!(rel_err(X_FACTOR_C, computed_xfc) < 1e-14,
            "X_FACTOR_C mismatch: {} vs {}", X_FACTOR_C, computed_xfc);

        // K_FACTOR_C = 3/10*(6*pi^2)^(2/3)
        let computed_kfc = 0.3 * (6.0 * std::f64::consts::PI * std::f64::consts::PI).powf(2.0 / 3.0);
        assert!(rel_err(K_FACTOR_C, computed_kfc) < 1e-14,
            "K_FACTOR_C mismatch: {} vs {}", K_FACTOR_C, computed_kfc);

        // KF_CONST = (3*pi^2)^(1/3)
        let computed_kf = (3.0 * std::f64::consts::PI * std::f64::consts::PI).cbrt();
        assert!(rel_err(KF_CONST, computed_kf) < 1e-15,
            "KF_CONST mismatch: {} vs {}", KF_CONST, computed_kf);

        // FZETAFACTOR = 2^(4/3) - 2
        let computed_fzf = 2.0_f64.powf(4.0 / 3.0) - 2.0;
        assert!(rel_err(FZETAFACTOR, computed_fzf) < 1e-14,
            "FZETAFACTOR mismatch: {} vs {}", FZETAFACTOR, computed_fzf);
    }

    #[test]
    fn test_rs_const_is_rs_factor() {
        assert_eq!(RS_CONST, RS_FACTOR);
    }
}
