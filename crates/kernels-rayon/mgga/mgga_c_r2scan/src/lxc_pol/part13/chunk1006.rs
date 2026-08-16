//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1006/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1006(t2574: f64, t3308: f64, t10776: f64, t2578: f64, t10772: f64, t10843: f64, t11782: f64, t11785: f64, t11788: f64, t11791: f64, t11795: f64, t11798: f64, t11800: f64, t11803: f64, t11806: f64) -> (f64, f64, f64) {
    let t11808 = t3308 * t2574;
    let t11809 = t10776 * t11808;
    let t11811 = t3308 * t2578;
    let t11812 = t10772 * t11811;
    let t11814 = t10843 - 0.21831846657716620896e-2_f64 * t11782 + 0.21831846657716620896e-2_f64 * t11785 + 0.65495539973149862688e-2_f64 * t11788 + 0.21831846657716620896e-2_f64 * t11791 + 0.21831846657716620896e-2_f64 * t11795 - 0.43341108700271342816e-1_f64 * t11798 - 0.27439371595564631661e-1_f64 * t11800 + 0.43341108700271342816e-1_f64 * t11803 + 0.13002332610081402845e0_f64 * t11806 + 0.43341108700271342816e-1_f64 * t11809 + 0.13002332610081402845e0_f64 * t11812;
    (t11808, t11811, t11814)
}
