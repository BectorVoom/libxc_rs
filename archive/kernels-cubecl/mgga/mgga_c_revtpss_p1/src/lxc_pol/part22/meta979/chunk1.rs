//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3291/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3291<F: Float>(t231: F, t62347: F, t62383: F, t18616: F, t221: F, t2484: F, t2485: F, t10815: F, t5980: F, t40398: F, t6024: F, t18435: F) -> (F, F, F, F, F) {
    let t62385 = (t62347 + t62383) * t231;
    let t62392 = t2484 * t2485 * t221 * t18616;
    let t62399 = t10815 * t5980;
    let t62401 = t40398 * t6024;
    let t62403 = t221 * t18435;
    (t62385, t62392, t62399, t62401, t62403)
}
