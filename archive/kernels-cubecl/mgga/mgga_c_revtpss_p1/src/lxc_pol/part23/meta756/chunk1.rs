//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2547/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2547<F: Float>(t3298: F, t4746: F, t4891: F, t12012: F, t15822: F, t1086: F, t15654: F, t3090: F, t1025: F, t371: F, t4852: F, t676: F) -> (F, F, F, F) {
    let t53800 = t4746 * t3298 * t4891;
    let t53807 = t15822 * t12012;
    let t53855 = t15654 * t1086 * t3090;
    let t53875 = t1025 * t371 * t676 * t4852;
    (t53800, t53807, t53855, t53875)
}
