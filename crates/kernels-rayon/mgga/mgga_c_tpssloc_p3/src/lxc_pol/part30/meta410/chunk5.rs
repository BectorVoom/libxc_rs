//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1555/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1555(t25: f64, t265: f64, t394: f64, t17133: f64, t18173: f64, t18174: f64, t1074: f64, t1408: f64, t1409: f64, t1642: f64, t16557: f64, t16558: f64, t17141: f64, t396: f64, t3966: f64, t40: f64, t4324: f64, t4705: f64, t5397: f64, t5398: f64, t5669: f64, t5955: f64, t606: f64, t607: f64, t873: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t18176 = piecewise3(t395, t18173 + t18174, t17133);
    let t18188 = piecewise3(t115, t17133 * t25 / 2.0_f64 + t5669 * t606 / 2.0_f64 + t4324 * t1408 + t17141 + t873 * t5397 / 2.0_f64 + t265 * t16557 / 2.0_f64, t18176 * t40 / 2.0_f64 + t5955 * t607 / 2.0_f64 + t4705 * t1409 + t1642 * t3966 + t1074 * t5398 / 2.0_f64 + t396 * t16558 / 2.0_f64);
    t18188
}
