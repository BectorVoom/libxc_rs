//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 970/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk970(t30141: f64, t2356: f64, t9296: f64, t10339: f64, t10342: f64, t10351: f64, t12815: f64, t30124: f64, t30127: f64, t30129: f64, t30133: f64, t30135: f64, t30140: f64) -> f64 {
    let t30142 = 3.0_f64 / 8.0_f64 * t30141;
    let t30143 = t2356 * t9296;
    let t30144 = 3.0_f64 / 16.0_f64 * t30143;
    let t30145 = -t30124 - t10339 + t10342 - t30127 - t30129 + t10351 + t30133 - t30135 - t30140 + t30142 - t12815 + t30144;
    t30145
}
