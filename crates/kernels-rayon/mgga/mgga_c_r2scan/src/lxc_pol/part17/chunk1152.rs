//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1152/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1152(t42877: f64, t481: f64, t792: f64, t795: f64, t797: f64, t9560: f64, t114: f64, t97: f64, t2847: f64, t3574: f64, t12570: f64, t31393: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42878 = t42877 * t481;
    let t42882 = t42877 * t792;
    let t42886 = t42877 * t795;
    let t42901 = t797 * t9560;
    let t42916 = t97 * t481 * t114;
    let t42919 = t3574 * t2847;
    let t42934 = t12570 * t481;
    let t42940 = t31393 * t795;
    (t42878, t42882, t42886, t42901, t42916, t42919, t42934, t42940)
}
