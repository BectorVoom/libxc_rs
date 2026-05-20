//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3290/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3290<F: Float>(t2475: F, t5962: F, t10696: F, t5966: F, t14468: F, t14643: F, t14649: F, t14653: F, t14656: F, t18392: F, t18586: F, t18592: F, t18599: F, t18600: F, t18603: F, t18608: F, t18609: F, t2394: F, t2430: F, t4415: F, t4416: F, t775: F, t833: F, t853: F) -> (F, F, F) {
    let t62351 = t2475 * t5962;
    let t62361 = t10696 * t5966;
    let t62383 = -F::new(24.0) * t18392 * t4415 * t775 * t853 - F::new(24.0) * t14468 * t4415 * t4416 + F::new(60.0) * t18599 * t2430 * t4415 - F::new(12.0) * t18608 * t2430 * t4415 + F::new(60.0) * t2394 * t4415 * t62351 - F::new(360.0) * t2394 * t4415 * t62361 + F::new(120.0) * t14643 * t18600 - F::new(48.0) * t14643 * t18603 - F::new(24.0) * t14643 * t18609 + F::new(120.0) * t14649 * t18592 - F::new(48.0) * t14653 * t18592 - F::new(24.0) * t14656 * t18592 + F::new(6.0) * t18586 * t833;
    (t62351, t62361, t62383)
}
