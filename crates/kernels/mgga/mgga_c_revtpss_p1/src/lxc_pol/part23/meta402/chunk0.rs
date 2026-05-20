//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1771/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1771<F: Float>(t1774: F, t3617: F, t372: F, t5268: F, t473: F, t5412: F, t13147: F, t487: F, t460: F, t12050: F, t13045: F, t13141: F) -> (F, F, F, F, F, F, F) {
    let t17794 = t3617 * t1774;
    let t17799 = t372 * t5268;
    let t17821 = t473 * t5412;
    let t17845 = t13147 * t487;
    let t17846 = t460 * t17845;
    let t17847 = t12050 * t13045;
    let t17852 = t13141 * t487;
    (t17794, t17799, t17821, t17845, t17846, t17847, t17852)
}
