//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 882/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk882(t39300: f64, t7411: f64, t1240: f64, t236: f64, t618: f64, t7230: f64, t7231: f64, t2305: f64, t35326: f64, t7371: f64, t8577: f64, t39277: f64, t7234: f64) -> (f64, f64, f64, f64, f64) {
    let t39301 = t39300 * t7411;
    let t39306 = t7230 * t7231 * t236 * t618 * t1240;
    let t39308 = t35326 * t2305;
    let t39310 = t8577 * t7371;
    let t39312 = t39277 * t7234;
    (t39301, t39306, t39308, t39310, t39312)
}
