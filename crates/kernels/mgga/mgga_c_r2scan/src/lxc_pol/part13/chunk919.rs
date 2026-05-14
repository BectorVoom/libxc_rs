//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 919/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk919<F: Float>(t2559: F, t3295: F, t2563: F, t3308: F, t10776: F, t2568: F, t10772: F, t2574: F, t2578: F, t10843: F, t11782: F, t11785: F, t11788: F, t11791: F, t11795: F, t11798: F) -> (F, F, F, F, F) {
    let t11800 = t3295 * t2559;
    let t11802 = t3308 * t2563;
    let t11803 = t10776 * t11802;
    let t11805 = t3308 * t2568;
    let t11806 = t10772 * t11805;
    let t11808 = t3308 * t2574;
    let t11809 = t10776 * t11808;
    let t11811 = t3308 * t2578;
    let t11812 = t10772 * t11811;
    let t11814 = t10843 - 0.21831846657716620896e-2 * t11782 + 0.21831846657716620896e-2 * t11785 + 0.65495539973149862688e-2 * t11788 + 0.21831846657716620896e-2 * t11791 + 0.21831846657716620896e-2 * t11795 - 0.43341108700271342816e-1 * t11798 - 0.27439371595564631661e-1 * t11800 + 0.43341108700271342816e-1 * t11803 + 0.13002332610081402845e0 * t11806 + 0.43341108700271342816e-1 * t11809 + 0.13002332610081402845e0 * t11812;
    (t11802, t11805, t11808, t11811, t11814)
}
