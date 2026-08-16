//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 904/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk904(t1224: f64, t28373: f64, t4840: f64, t28377: f64, t1697: f64, t28381: f64, t28385: f64, t28389: f64, t11105: f64, t17382: f64, t23460: f64, t23472: f64, t23481: f64, t29082: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29085 = t1224 * t4840 * t28373;
    let t29088 = t1224 * t4840 * t28377;
    let t29091 = t1224 * t1697 * t28381;
    let t29094 = t1224 * t1697 * t28385;
    let t29097 = t1224 * t1697 * t28389;
    let t29099 = -t11105 - 0.23744444444444444444e-1_f64 * t17382 + 0.11872222222222222222e-1_f64 * t23460 - 0.35616666666666666666e-1_f64 * t23472 + 0.17808333333333333333e-1_f64 * t23481 - 0.19787037037037037037e-1_f64 * t29082 + 0.71233333333333333332e-1_f64 * t29085 - 0.35616666666666666666e-1_f64 * t29088 - 0.10685e0_f64 * t29091 + 0.10685e0_f64 * t29094 - 0.17808333333333333333e-1_f64 * t29097;
    (t29085, t29088, t29091, t29094, t29097, t29099)
}
