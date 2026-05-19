//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 706/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk706<F: Float>(t9636: F, t626: F, t705: F, t701: F, t173: F, t2442: F, t2447: F, t2451: F, t191: F, t2360: F, t9570: F, t9571: F) -> (F, F, F, F, F, F, F) {
    let t9637 = F::cast_from(0.70937342644032921812e-2_f64) * t9636;
    let t9638 = t626 * t705;
    let t9639 = t701 * t9638;
    let t9641 = t173 * t2442;
    let t9642 = t701 * t9641;
    let t9644 = t173 * t2447;
    let t9645 = t701 * t9644;
    let t9647 = t173 * t2451;
    let t9648 = t701 * t9647;
    let t9651 = F::new(1.0) / t191 / t2360;
    let t9652 = t9651 * t9570;
    let t9653 = t9652 * t9571;
    (t9637, t9639, t9642, t9645, t9648, t9651, t9653)
}
