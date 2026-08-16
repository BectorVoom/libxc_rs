//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2084/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2084<F: Float>(t4786: F, t4873: F, t15584: F, t11696: F, t4781: F, t3092: F, t11705: F, t11703: F, t11678: F, t357: F, t1592: F, t4900: F, t999: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15585 = t4873 * t4786;
    let t15586 = t15584 * t15585;
    let t15591 = t4781 * t11696;
    let t15592 = t3092 * t15591;
    let t15595 = t4781 * t11705;
    let t15596 = t11703 * t15595;
    let t15599 = t11678 * t357;
    let t15600 = t1592 * t15599;
    let t15601 = t3092 * t15600;
    let t15604 = t4900 * t999;
    (t15585, t15586, t15591, t15592, t15595, t15596, t15599, t15600, t15601, t15604)
}
