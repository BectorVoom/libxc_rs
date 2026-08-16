//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2667/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2667<F: Float>(t5854: F, t607: F, t10355: F, t5819: F, t606: F, t4186: F, t4201: F, t2275: F, t5825: F, t18281: F, t48: F, t10368: F) -> (F, F, F, F, F, F, F, F) {
    let t21727 = t607 * t5854;
    let t21732 = t10355 * t5819;
    let t21733 = t21732 * t606;
    let t21736 = t4201 * t4186;
    let t21741 = t2275 * t5825;
    let t21742 = t21741 * t606;
    let t21745 = t48 * t18281;
    let t21754 = t10368 * t5819;
    (t21727, t21732, t21733, t21736, t21741, t21742, t21745, t21754)
}
