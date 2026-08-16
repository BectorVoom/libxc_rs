//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 509/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk509<F: Float>(t1131: F, t2: F, t2372: F, t713: F, t192: F, t3821: F, t743: F, t2481: F, t2482: F, t2484: F, t3139: F, t3908: F, t3911: F, t3914: F, t3918: F, t3922: F, t3925: F, t3927: F, t462: F, t92: F) -> (F, F, F, F) {
    let t3930 = t2 * t1131;
    let t3932 = t2372 * t3930 * t713;
    let t3936 = t192 * t743 * t3821;
    let t3938 = t2481 + t2482 / F::cast_from(9.0_f64) + t2484 / F::cast_from(3.0_f64) + t3908 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t462 * t3911 + t462 * t3914 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t3918 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3139 * t3922 + t3925 / F::cast_from(3.0_f64) + t462 * t3927 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t462 * t3932 - t92 * t3936;
    (t3930, t3932, t3936, t3938)
}
