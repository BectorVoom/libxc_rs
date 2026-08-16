//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2897/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2897<F: Float>(t41361: F, t41363: F, t41369: F, t51978: F, t51981: F, t51984: F, t51987: F, t51990: F, t51995: F, t52000: F, t52004: F, t52035: F) -> (F, F) {
    let t52588 = F::cast_from(0.53560370370370370369e0_f64) * t51978 - F::cast_from(0.10805407407407407407e0_f64) * t51981 + F::cast_from(0.62517e0_f64) * t51984 + F::cast_from(0.20839e0_f64) * t51987 + F::cast_from(0.62517e0_f64) * t51990 + F::cast_from(0.62517000000000000001e0_f64) * t51995 + F::cast_from(0.55570666666666666666e0_f64) * t52000 - F::cast_from(0.187551e1_f64) * t52004 + F::cast_from(0.16068111111111111111e1_f64) * t41361 + F::cast_from(0.13772666666666666666e1_f64) * t41363 - F::cast_from(0.68863333333333333332e0_f64) * t41369;
    let t52597 = F::cast_from(0.13772666666666666666e1_f64) * t52035;
    (t52588, t52597)
}
