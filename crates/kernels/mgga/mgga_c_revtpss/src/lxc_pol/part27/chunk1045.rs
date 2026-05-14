//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1045/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1045<F: Float>(t670: F, t7683: F, t2163: F, t2371: F, t2127: F, t2165: F, t2372: F, t25193: F, t25196: F, t25804: F, t25838: F, t25840: F, t25842: F, t25844: F, t25846: F, t25853: F, t25858: F, t25860: F, t25863: F, t25868: F, t26091: F, t27060: F, t3813: F, t4151: F, t651: F, t671: F, t7586: F) -> (F, F, F) {
    let t27076 = t7683 * t670;
    let t27079 = t2163 * t2371;
    let t27088 = -t2127 * t3813 + t2165 * t4151 - 2.0 * t2372 * t7586 - 4.0 * t27060 * t671 - 4.0 * t27076 * t651 - 2.0 * t27079 * t651 + t25193 - t25196 - t25804 + t25838 - t25840 - t25842 - t25844 + t25846 - t25853 - t25858 - t25860 - t25863 + t25868 + t26091;
    (t27076, t27079, t27088)
}
