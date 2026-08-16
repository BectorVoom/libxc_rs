//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 654/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk654(t118: f64, t2000: f64, t1004: f64, t107: f64, t490: f64, t1326: f64, t1330: f64, t31: f64, t356: f64, t640: f64, t2164: f64, t7556: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35039 = t2000 * t118;
    let t35154 = t1004 * t107;
    let t35155 = t490 * t35154;
    let t35206 = t1326 * t1330;
    let t35219 = t356 * t31;
    let t35228 = t640 * t35219;
    let t35244 = t2164 * t7556;
    (t35039, t35154, t35155, t35206, t35228, t35244)
}
