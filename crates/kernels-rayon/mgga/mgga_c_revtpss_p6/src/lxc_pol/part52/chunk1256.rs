//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1256/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1256(t34222: f64, t686: f64, t72: f64, t32705: f64, t32710: f64, t125617: f64, t121000: f64, t121004: f64, t122321: f64, t122327: f64, t122331: f64, t1444: f64, t32250: f64, t32700: f64, t34223: f64, t34227: f64, t7506: f64, t7910: f64, t8706: f64, t8707: f64) -> f64 {
    let t128628 = t34222 * t72 * t686;
    let t128629 = t32705 * t128628;
    let t128631 = t32710 * t128628;
    let t128644 = 0.263521689745817692e-2_f64 * t125617;
    let t128647 = t122321 + 0.26447628533477078895e-3_f64 * t121000 + t121004 - 0.14279934416275588154e-1_f64 * t128629 + 0.25389723392137995738e-1_f64 * t128631 + 0.14456046980341999104e-1_f64 * t122327 + t122331 + 0.57119737665102352616e0_f64 * t32700 * t34227 + 0.57119737665102352616e0_f64 * t8706 * t8707 * t7506 * t7910 - 0.17135921299530705785e1_f64 * t8706 * t32250 * t34222 * t1444 + t128644 + 0.57119737665102352616e0_f64 * t32700 * t34223;
    t128647
}
