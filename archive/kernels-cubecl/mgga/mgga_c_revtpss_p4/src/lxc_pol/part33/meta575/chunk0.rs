//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1984/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1984<F: Float>(t1398: F, t6861: F, t221: F, t22274: F, t22279: F, t22287: F, t6843: F, t1883: F, t5658: F, t543: F, t73820: F, t6862: F) -> (F, F, F, F, F, F, F, F) {
    let t73842 = t6861 * t1398;
    let t74419 = t221 * t22274;
    let t74423 = t221 * t22279;
    let t74477 = t221 * t22287;
    let t74700 = t6843 * t1398;
    let t75012 = t1883 * t5658;
    let t75016 = t73842 * t543;
    let t75047 = t73820 * t1398;
    let t75051 = t6862 * t1398;
    (t74419, t74423, t74477, t74700, t75012, t75016, t75047, t75051)
}
