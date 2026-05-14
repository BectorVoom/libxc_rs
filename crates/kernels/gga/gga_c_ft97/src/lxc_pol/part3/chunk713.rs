//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 713/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk713<F: Float>(t16023: F, t16081: F, t16144: F, t16194: F, t16227: F, t16284: F, t16332: F, t16544: F, t108: F, t15594: F, t15596: F, t15599: F, t15968: F, t2976: F, t3109: F, t3289: F, t438: F, t4415: F, t4501: F, t4621: F, t497: F, t88: F, t948: F, t984: F) -> (F,) {
    let t16547 = t16023 + t16081 + t16144 + t16194 + t16227 + t16284 + t16332 + t16544;
    let t16549 = -t108 * t15594 - t108 * t15596 - t108 * t15599 - t108 * t15968 - t16547 * t88 - 2.0 * t2976 * t984 - 2.0 * t3109 * t984 - 2.0 * t3289 * t948 - t438 * t4621 - t4415 * t497 - t4501 * t497;
    (t16549,)
}
