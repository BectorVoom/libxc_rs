//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 232/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk232<F: Float>(t378: F, t994: F, t225: F, t385: F, t902: F, t908: F) -> (F, F, F, F) {
    let t995 = t994 * t378;
    let t996 = t225 * t385;
    let t997 = F::new(0.14816666666666666667e-1) * t902;
    let t999 = -t997 - F::new(0.14816666666666666667e-1) * t908;
    (t995, t996, t997, t999)
}
