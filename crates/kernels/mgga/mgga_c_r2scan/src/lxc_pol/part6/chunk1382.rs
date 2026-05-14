//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1382/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1382<F: Float>(t783: F, t784: F, t788: F, t7916: F, t2837: F, t5103: F, t5104: F, t6425: F, t8176: F, t2592: F, t5147: F, t5148: F, t20622: F, t928: F, t6235: F, t8240: F) -> (F, F, F, F, F, F) {
    let t26218 = t783 * t7916 * t784 * t788;
    let t26221 = t5103 * t2837 * t5104;
    let t26231 = t6425 * t8176;
    let t26234 = t5147 * t5148 * t2592;
    let t26238 = t20622 * t928;
    let t26244 = t8240 * t6235;
    (t26218, t26221, t26231, t26234, t26238, t26244)
}
