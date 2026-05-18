//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 851/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk851<F: Float>(t2086: F, t4778: F, t590: F, t91: F, t4753: F, t9252: F, t3491: F, t3526: F, t16710: F, t16714: F, t16717: F, t16721: F, t16724: F, t16727: F, t16730: F, t16734: F) -> (F, F, F, F) {
    let t17235 = t2086 * t4778;
    let t17237 = t91 * t17235 * t590;
    let t17239 = t9252 * t4753;
    let t17241 = t91 * t17239 * t590;
    let t17244 = t91 * t3491 * t3526;
    let t17246 = F::new(4.0) / F::new(3.0) * t16710 - F::new(2.0) / F::new(3.0) * t16714 - F::new(2.0) * t16717 + F::new(2.0) / F::new(9.0) * t16721 + F::new(4.0) / F::new(3.0) * t16724 - F::new(10.0) / F::new(27.0) * t16727 - F::new(8.0) / F::new(9.0) * t16730 + F::new(2.0) / F::new(3.0) * t16734 - t17237 / F::new(4.0) + F::new(3.0) / F::new(8.0) * t17241 - t17244 / F::new(2.0);
    (t17237, t17241, t17244, t17246)
}
