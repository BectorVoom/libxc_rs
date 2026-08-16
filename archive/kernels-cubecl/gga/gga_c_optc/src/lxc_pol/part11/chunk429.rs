//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 429/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk429<F: Float>(t2257: F, t248: F, t808: F, t243: F, t2280: F) -> (F, F, F, F, F, F, F) {
    let t2480 = F::cast_from(0.22831111111111111111e-1_f64) * t2257;
    let t2491 = t808 * t248;
    let t2492 = F::cast_from(1.0_f64) / t2491;
    let t2493 = t243 * t2492;
    let t2500 = F::cast_from(0.68863333333333333333e0_f64) * t2257;
    let t2507 = F::cast_from(0.17365833333333333333e0_f64) * t2280;
    let t2516 = t808 * t808;
    (t2480, t2491, t2492, t2493, t2500, t2507, t2516)
}
