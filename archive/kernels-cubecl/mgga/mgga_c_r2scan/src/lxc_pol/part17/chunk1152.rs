//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1152/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1152<F: Float>(t42877: F, t481: F, t792: F, t795: F, t797: F, t9560: F, t114: F, t97: F, t2847: F, t3574: F, t12570: F, t31393: F) -> (F, F, F, F, F, F, F, F) {
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
