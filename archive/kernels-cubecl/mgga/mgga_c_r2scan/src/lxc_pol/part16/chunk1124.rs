//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1124/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1124<F: Float>(t1065: F, t2526: F, t3270: F, t105: F, t2530: F, t97: F, t3617: F, t5086: F, t11855: F, t1561: F, t113: F, t3578: F) -> (F, F, F, F, F) {
    let t40676 = t1065 * t2526;
    let t40677 = t3270 * t40676;
    let t40681 = t97 * t105 * t2530;
    let t40687 = t5086 * t3617;
    let t40705 = t1561 * t11855;
    let t40713 = t97 * t3578 * t113;
    (t40677, t40681, t40687, t40705, t40713)
}
