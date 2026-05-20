//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2623/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2623<F: Float>(t18263: F, t2615: F, t2475: F, t5962: F, t10696: F, t5966: F, t18616: F, t221: F, t2484: F, t2485: F, t10815: F, t5980: F) -> (F, F, F, F, F) {
    let t62302 = t18263 * t2615;
    let t62351 = t2475 * t5962;
    let t62361 = t10696 * t5966;
    let t62392 = t2484 * t2485 * t221 * t18616;
    let t62399 = t10815 * t5980;
    (t62302, t62351, t62361, t62392, t62399)
}
