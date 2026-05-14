//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1289/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1289<F: Float>(t7339: F, t9507: F, t6334: F, t921: F, t20957: F, t7338: F, t1551: F, t2533: F, t2123: F, t6375: F, t6363: F, t920: F, t5068: F, t20318: F, t6118: F, t7357: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24188 = t9507 * t7339;
    let t24192 = t921 * t6334;
    let t24196 = t7338 * t20957;
    let t24204 = t2533 * t1551;
    let t24208 = t2123 * t6375;
    let t24209 = t920 * t6363;
    let t24210 = t24209 * t5068;
    let t24214 = t24209 * t20318;
    let t24218 = t6118 * t7357;
    (t24188, t24192, t24196, t24204, t24208, t24209, t24210, t24214, t24218)
}
