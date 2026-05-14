//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 725/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk725<F: Float>(t18712: F, t3885: F, t2606: F, t3892: F, t3891: F, t10085: F, t5166: F, t3821: F, t992: F, t2600: F, t2599: F, t258: F, t5053: F, t684: F, t14159: F, t3898: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t18713 = t3885 * t18712;
    let t18714 = t2606 * t18713;
    let t18717 = t3892 * t18712;
    let t18718 = t3891 * t18717;
    let t18721 = t10085 * t5166;
    let t18724 = t992 * t3821;
    let t18725 = t2600 * t18724;
    let t18726 = t2599 * t18725;
    let t18729 = t258 * t5053;
    let t18730 = t18729 * t684;
    let t18731 = t2599 * t18730;
    let t18734 = t14159 * t3898;
    (t18713, t18714, t18717, t18718, t18721, t18724, t18725, t18726, t18730, t18731, t18734)
}
