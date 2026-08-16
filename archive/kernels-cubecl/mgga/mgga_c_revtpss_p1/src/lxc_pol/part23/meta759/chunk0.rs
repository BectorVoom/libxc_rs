//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2552/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2552<F: Float>(t43347: F, t53668: F, t11852: F, t41270: F, t3316: F, t4746: F, t4891: F, t16381: F, t3090: F, t11262: F, t3127: F, t4874: F) -> (F, F, F, F, F) {
    let t54509 = t43347 * t53668;
    let t54537 = t11852 * t41270;
    let t54570 = t4746 * t3316 * t4891;
    let t54578 = t16381 * t3090;
    let t54599 = t3127 * t11262 * t4874;
    (t54509, t54537, t54570, t54578, t54599)
}
