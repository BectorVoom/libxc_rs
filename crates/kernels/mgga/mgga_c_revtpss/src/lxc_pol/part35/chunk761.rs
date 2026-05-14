//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 761/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk761<F: Float>(t18688: F, t869: F, t689: F, t251: F, t6016: F, t6041: F, t822: F, t6022: F, t72: F, t686: F, t10530: F, t6017: F, t2798: F, t5978: F, t14568: F, t4500: F) -> (F, F, F, F, F, F, F) {
    let t18689 = t869 * t18688;
    let t18690 = t689 * t18689;
    let t18699 = t251 * t6016;
    let t18714 = t822 * t6041;
    let t18718 = t6022 * t72;
    let t18719 = t18718 * t686;
    let t18720 = t10530 * t18719;
    let t18725 = t6017 * t72;
    let t18726 = t18725 * t686;
    let t18727 = t2798 * t18726;
    let t18729 = t5978 * t72;
    let t18730 = t18729 * t686;
    let t18731 = t2798 * t18730;
    let t18733 = t14568 * t4500;
    (t18690, t18699, t18714, t18720, t18727, t18731, t18733)
}
