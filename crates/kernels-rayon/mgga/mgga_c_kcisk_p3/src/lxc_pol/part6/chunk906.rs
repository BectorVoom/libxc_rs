//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 906/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk906(t2408: f64, t8701: f64, t11036: f64, t11056: f64, t11040: f64, t17382: f64, t23460: f64, t23472: f64, t23481: f64, t29082: f64, t29085: f64, t29088: f64, t29091: f64, t29094: f64, t29097: f64) -> (f64, f64, f64) {
    let t29123 = t8701 * t2408;
    let t29124 = t11036 * t29123;
    let t29126 = t11056 * t29123;
    let t29138 = -t11040 - 4.0_f64 / 9.0_f64 * t17382 + 2.0_f64 / 9.0_f64 * t23460 - 2.0_f64 / 3.0_f64 * t23472 + t23481 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t29082 + 4.0_f64 / 3.0_f64 * t29085 - 2.0_f64 / 3.0_f64 * t29088 - 2.0_f64 * t29091 + 2.0_f64 * t29094 - t29097 / 3.0_f64;
    (t29124, t29126, t29138)
}
