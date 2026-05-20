//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3115/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3115<F: Float>(t11223: F, t16088: F, t380: F, t1041: F, t16185: F, t3172: F, t1062: F, t42261: F, t11710: F, t15974: F, t4899: F, t11866: F, t15794: F) -> (F, F, F, F, F) {
    let t54857 = t11223 * t380 * t16088;
    let t54869 = t1041 * t3172 * t16185;
    let t54899 = t42261 * t1062;
    let t54907 = t4899 * t11710 * t15974;
    let t54914 = t11866 * t15794;
    (t54857, t54869, t54899, t54907, t54914)
}
