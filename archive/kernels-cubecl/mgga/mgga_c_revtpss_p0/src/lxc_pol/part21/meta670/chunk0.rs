//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2471/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2471<F: Float>(t12166: F, t989: F, t16409: F, t994: F, t3057: F, t4980: F, t11223: F, t3286: F, t11200: F, t11213: F, t3046: F, t4995: F) -> (F, F, F, F, F, F, F) {
    let t43420 = t989 * t12166;
    let t43432 = t994 * t16409;
    let t43438 = t3057 * t4980;
    let t43443 = t11223 * t3286;
    let t43446 = t11200 * t3286;
    let t43450 = t11213 * t3286;
    let t43453 = t3046 * t4995;
    (t43420, t43432, t43438, t43443, t43446, t43450, t43453)
}
