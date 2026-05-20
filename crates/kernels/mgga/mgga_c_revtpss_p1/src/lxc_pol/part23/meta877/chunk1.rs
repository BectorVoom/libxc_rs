//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2783/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2783<F: Float>(t49503: F, t5722: F, t213: F, t22307: F, t1358: F, t2439: F, t6888: F, t785: F, t1357: F, t22387: F, t689: F, t3899: F, t6896: F) -> (F, F, F, F, F) {
    let t74797 = t49503 * t5722;
    let t74802 = t213 * t22307;
    let t74807 = t2439 * t785 * t6888 * t1358;
    let t74810 = t689 * t1357 * t22387;
    let t74813 = t689 * t3899 * t6896;
    (t74797, t74802, t74807, t74810, t74813)
}
