//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1144/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1144<F: Float>(t20860: F, t546: F, t6477: F, t565: F, t6482: F, t110: F, t4145: F, t524: F, t531: F, t147: F, t5134: F, t5137: F, t785: F, t788: F, t1567: F, t2078: F) -> (F, F, F, F, F, F, F, F) {
    let t20861 = t546 * t20860;
    let t20862 = t20861 * t6477;
    let t20864 = t565 * t20860;
    let t20865 = t20864 * t6482;
    let t20868 = t524 * t4145 * t110;
    let t20869 = t20868 * t531;
    let t20871 = t5134 * t147;
    let t20874 = t20871 * t785 * t788 * t5137;
    let t20881 = t1567 * t2078;
    (t20861, t20862, t20864, t20865, t20868, t20869, t20874, t20881)
}
