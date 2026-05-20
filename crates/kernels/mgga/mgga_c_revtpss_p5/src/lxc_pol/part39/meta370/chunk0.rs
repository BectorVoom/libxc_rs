//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1299/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1299<F: Float>(t11705: F, t4781: F, t11703: F, t11678: F, t357: F, t1592: F, t3092: F, t4900: F, t999: F, t4893: F, t3117: F, t4894: F) -> (F, F, F, F, F) {
    let t15595 = t4781 * t11705;
    let t15596 = t11703 * t15595;
    let t15599 = t11678 * t357;
    let t15600 = t1592 * t15599;
    let t15601 = t3092 * t15600;
    let t15604 = t4900 * t999;
    let t15605 = t4893 * t15604;
    let t15606 = t3117 * t15605;
    let t15609 = t4894 * t999;
    (t15596, t15601, t15604, t15606, t15609)
}
