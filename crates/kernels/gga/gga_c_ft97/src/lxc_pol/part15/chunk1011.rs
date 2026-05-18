//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1011/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1011<F: Float>(t4449: F, t4466: F, t20049: F, t930: F, t4491: F, t19978: F, t938: F, t1594: F, t1624: F, t1631: F, t20050: F, t20090: F, t372: F, t374: F, t37835: F, t4467: F, t534: F, t7906: F, t7914: F) -> (F, F, F) {
    let t85608 = t4449 * t4466;
    let t85618 = t930 * t20049;
    let t85626 = t4449 * t4491;
    let t85630 = t19978 * t938;
    let t85644 = F::new(0.279058811357253504e0) * t37835 * t374 * t930 * t20090 - F::new(0.19352371901929178119e-4) * t372 * t534 * t85608 - F::new(0.1674352868143521024e-1) * t372 * t7914 * t85608 - F::new(0.69716604262587839785e-3) * t372 * t7906 * t85608 + F::new(0.93019603785751168e-2) * t372 * t1631 * t85618 + F::new(0.69764702839313376e-1) * t1624 * t374 * t4467 * t4491 - F::new(0.11619434043764639964e-2) * t1624 * t1594 * t85626 + F::new(0.46477736175058559857e-3) * t1624 * t7906 * t85630 + F::new(0.12901581267952785412e-4) * t1624 * t534 * t85630 - F::new(0.139529405678626752e-1) * t1624 * t1631 * t85626 + F::new(0.46509801892875584e-1) * t1624 * t374 * t20050 * t938;
    (t85618, t85630, t85644)
}
