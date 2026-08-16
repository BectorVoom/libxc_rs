//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1085/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1085(t10328: f64, t15799: f64, t15800: f64, t15803: f64, t15804: f64, t30145: f64, t31807: f64, t5581: f64, t7725: f64, t7730: f64, t7733: f64, t8: f64, t8461: f64, t8465: f64, t8467: f64, t8469: f64, t8474: f64, t8477: f64, t9297: f64) -> f64 {
    let tv3rho33 = t8 * (t30145 + t31807) - t10328 + t15799 - 3.0_f64 / 16.0_f64 * t7730 - 3.0_f64 / 8.0_f64 * t7733 - 3.0_f64 / 16.0_f64 * t8461 + t15800 - 3.0_f64 / 8.0_f64 * t8465 + 3.0_f64 * t7725 - 3.0_f64 / 16.0_f64 * t9297 - t15803 + t15804 - 3.0_f64 / 16.0_f64 * t8474 - 3.0_f64 / 8.0_f64 * t8477 + 3.0_f64 / 8.0_f64 * t8467 + 3.0_f64 / 8.0_f64 * t8469 + 6.0_f64 * t5581;
    tv3rho33
}
