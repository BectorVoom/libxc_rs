//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 499/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk499<F: Float>(t272: F, t2775: F, t686: F, t286: F, t224: F, t804: F, t277: F, t709: F, t244: F, t1255: F, t377: F, t457: F, t980: F, t313: F, t111: F, t150: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2987 = t686 * t2775 * t272;
    let t2988 = t286 * t2987;
    let t2989 = 0.35089341735807877242e1 * t2988;
    let t2992 = t224 * t804;
    let t2994 = t709 * t277;
    let t2995 = 60.0 * t2994;
    let t2998 = t709 * t244;
    let t3029 = t377 * t1255;
    let t3031 = t980 * t457;
    let t3033 = t313 * t313;
    let t3034 = 1.0 / t3033;
    let t3035 = t111 * t3034;
    let t3036 = t3035 * t150;
    (t2989, t2992, t2995, t2998, t3029, t3031, t3033, t3034, t3035, t3036)
}
