//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3155/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3155<F: Float>(t127: F, t12866: F, t17650: F, t5296: F, t17861: F, t3624: F, t12784: F, t17451: F, t17416: F, t3647: F, t11262: F, t1247: F, t5286: F) -> (F, F, F, F, F) {
    let t57098 = t12866 * t127 * t5296 * t17650;
    let t57100 = t17861 * t3624;
    let t57114 = t12784 * t17451;
    let t57118 = t3647 * t17416;
    let t57125 = t1247 * t11262 * t5286;
    (t57098, t57100, t57114, t57118, t57125)
}
