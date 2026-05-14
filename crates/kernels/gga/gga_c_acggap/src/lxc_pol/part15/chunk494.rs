//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 494/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk494<F: Float>(t286: F, t2981: F, t682: F, t883: F, t272: F, t2775: F, t686: F, t224: F, t804: F, t277: F, t709: F, t244: F, t715: F, t699: F, t457: F, t980: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2982 = t286 * t2981;
    let t2983 = 0.5848223622634646207e0 * t2982;
    let t2984 = t883 * t682;
    let t2987 = t686 * t2775 * t272;
    let t2988 = t286 * t2987;
    let t2989 = 0.35089341735807877242e1 * t2988;
    let t2992 = t224 * t804;
    let t2994 = t709 * t277;
    let t2995 = 60.0 * t2994;
    let t2996 = t715 * t244;
    let t2998 = t709 * t244;
    let t3000 = t224 * t699;
    let t3031 = t980 * t457;
    (t2983, t2984, t2989, t2992, t2995, t2996, t2998, t3000, t3031)
}
