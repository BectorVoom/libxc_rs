//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1433/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1433(t11972: f64, t1266: f64, t1393: f64, t22461: f64, t22483: f64, t22559: f64, t2314: f64, t2323: f64, t2364: f64, t23829: f64, t23855: f64, t3652: f64, t574: f64, t6515: f64, t6517: f64, t652: f64, t671: f64, t672: f64, t83896: f64, t83905: f64, t83913: f64, t83917: f64, t83919: f64, t83921: f64, t83924: f64, t83928: f64, t83932: f64, t83935: f64, t83939: f64, t83969: f64) -> f64 {
    let t83971 = -6.0_f64 * t23829 * t652 * t671 - 2.0_f64 * t11972 * t6517 - 3.0_f64 * t1266 * t22559 + 3.0_f64 * t1393 * t23855 - 12.0_f64 * t22461 * t2323 - 6.0_f64 * t22461 * t2364 - 6.0_f64 * t22483 * t2314 - 3.0_f64 * t3652 * t6515 + t574 * t83969 - 6.0_f64 * t672 * t83935 - t83896 + t83905 - t83913 - t83917 - t83919 - t83921 - t83924 - t83928 + t83932 - t83939;
    t83971
}
