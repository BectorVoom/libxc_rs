//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2947/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2947<F: Float>(t15905: F, t43420: F, t43574: F, t11922: F, t15781: F, t4892: F, t42865: F, t72: F, t3088: F, t43472: F, t1668: F, t42871: F) -> (F, F, F, F, F, F, F) {
    let t53654 = t43420 * t15905;
    let t53657 = t43574 * t15905;
    let t53661 = t4892 * t11922 * t15781;
    let t53667 = t42865 * t72;
    let t53668 = t3088 * t53667;
    let t53669 = t43472 * t53668;
    let t53670 = t1668 * t42871;
    (t53654, t53657, t53661, t53667, t53668, t53669, t53670)
}
