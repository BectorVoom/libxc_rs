//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 976/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk976(t25: f64, t265: f64, t394: f64, t10150: f64, t11098: f64, t11103: f64, t1074: f64, t2249: f64, t2250: f64, t2756: f64, t3220: f64, t396: f64, t40: f64, t606: f64, t607: f64, t873: f64, t9257: f64, t9258: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t11105 = piecewise3(t395, t11098 + t11103, t10150);
    let t11115 = piecewise3(t115, t10150 * t25 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2756 * t606 + 3.0_f64 / 2.0_f64 * t873 * t2249 + t265 * t9257 / 2.0_f64, t11105 * t40 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t3220 * t607 + 3.0_f64 / 2.0_f64 * t1074 * t2250 + t396 * t9258 / 2.0_f64);
    t11115
}
