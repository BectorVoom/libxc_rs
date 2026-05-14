//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1065/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1065<F: Float>(t4880: F, t4890: F, t4702: F, t4900: F, t2090: F, t7: F) -> (F, F, F, F, F) {
    let t18767 = 4.0 * t4880;
    let t18771 = 480.0 * t4890;
    let t18774 = 0.1929837539843104208e3 * t4702;
    let t18777 = 240.0 * t4900;
    let t18783 = t7 * t2090;
    (t18767, t18771, t18774, t18777, t18783)
}
