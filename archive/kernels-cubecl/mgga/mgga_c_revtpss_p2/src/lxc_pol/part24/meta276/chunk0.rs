//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1050/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1050<F: Float>(t233: F, t6041: F, t869: F, t689: F, t251: F, t6016: F, t822: F, t6022: F, t72: F, t686: F, t10530: F, t6017: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18688 = t233 * t6041;
    let t18689 = t869 * t18688;
    let t18690 = t689 * t18689;
    let t18699 = t251 * t6016;
    let t18714 = t822 * t6041;
    let t18718 = t6022 * t72;
    let t18719 = t18718 * t686;
    let t18720 = t10530 * t18719;
    let t18725 = t6017 * t72;
    (t18688, t18689, t18690, t18699, t18714, t18718, t18719, t18720, t18725)
}
