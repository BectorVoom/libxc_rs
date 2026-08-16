//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1096/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1096(t30: f64, t265: f64, t393: f64, t26625: f64, t2078: f64, t2258: f64, t26601: f64, t45: f64, t606: f64, t7449: f64, t1113: f64, t1940: f64, t2071: f64, t2403: f64, t25752: f64, t25760: f64, t25763: f64, t25767: f64, t25778: f64, t25781: f64, t25784: f64, t26425: f64, t26581: f64, t26585: f64, t26590: f64, t33: f64, t3351: f64, t4541: f64, t7200: f64, t7207: f64, t7428: f64, t7432: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t26626 = piecewise3(t394, 0.0_f64, t26625);
    let t26633 = piecewise3(t120, t26601, t26626 * t45 / 2.0_f64 + t7449 * t606 + t2078 * t2258 / 2.0_f64);
    let t26665 = 3.0_f64 * t4541 * t2071 * t25752 + 3.0_f64 * t2403 * t7428 * t7200 - 3.0_f64 * t26425 * t25760 + 3.0_f64 * t2403 * t2071 * t25763 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t25767 + t1940 * t26581 * t33 / 2.0_f64 - t1940 * t26585 * t7207 + t1940 * t7428 * t1113 + t1940 * t26590 * t25778 - t1940 * t7432 * t25781 - t1940 * t7432 * t25784 / 2.0_f64 + t1940 * t2071 * t3351 / 2.0_f64;
    (t26626, t26633, t26665)
}
