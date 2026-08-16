//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2146/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2146<F: Float>(t1079: F, t15578: F, t3215: F, t4858: F, t372: F, t4872: F, t4786: F, t4873: F, t11696: F, t4781: F, t3092: F, t11705: F) -> (F, F, F, F, F, F, F, F) {
    let t15579 = t1079 * t15578;
    let t15583 = F::cast_from(0.28582678745379824648e-3_f64) * t4858 * t3215;
    let t15584 = t372 * t4872;
    let t15585 = t4873 * t4786;
    let t15586 = t15584 * t15585;
    let t15591 = t4781 * t11696;
    let t15592 = t3092 * t15591;
    let t15595 = t4781 * t11705;
    (t15579, t15583, t15584, t15585, t15586, t15591, t15592, t15595)
}
