//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3284/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3284<F: Float>(t50873: F, t40172: F, t14330: F, t18575: F, t2258: F, t14370: F, t18259: F, t18562: F, t2626: F, t18576: F, t50895: F, t5819: F, t606: F, t749: F) -> (F, F, F, F, F, F, F) {
    let t62269 = F::new(16.0) * t50873;
    let t62270 = F::cast_from(0.20508037716432813316e4_f64) * t40172;
    let t62273 = F::new(24.0) * t14330 * t18575 * t2258;
    let t62274 = t18259 * t14370;
    let t62275 = F::new(48.0) * t62274;
    let t62276 = t18562 * t2626;
    let t62277 = F::cast_from(0.11696447245269292414e1_f64) * t62276;
    let t62279 = F::new(48.0) * t50895 * t18576;
    let t62282 = t14330 * t749 * t5819 * t606;
    (t62269, t62270, t62273, t62275, t62277, t62279, t62282)
}
