//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 260/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk260<F: Float>(t40: F, t902: F, t229: F, t277: F, t224: F, t244: F, t684: F, t693: F, t701: F, t703: F, t708: F, t764: F, t805: F, t885: F, t85: F, t901: F) -> (F, F, F, F) {
    let t903 = t40 * t902;
    let t904 = t229 * t277;
    let t905 = 8.0 * t904;
    let t906 = t224 * t244;
    let t907 = 8.0 * t906;
    let t908 = -t684 - t693 - t885 + t805 + t903 + t701 - t703 - t905 + t907 + t708 - t764;
    let t909 = t901 * t85;
    (t905, t906, t908, t909)
}
