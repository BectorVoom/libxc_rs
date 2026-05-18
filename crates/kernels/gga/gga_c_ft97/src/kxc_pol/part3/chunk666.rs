//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 666/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk666<F: Float>(t195: F, t9606: F, t25: F, t209: F, t2247: F, t228: F, t231: F, t626: F, t705: F, t701: F, t191: F, t2360: F) -> (F, F, F, F, F) {
    let t9608 = F::new(1.0) / t195 / t9606;
    let t9609 = t25 * t9608;
    let t9634 = t209 * t2247;
    let t9636 = t228 * t9634 * t231;
    let t9637 = F::new(0.70937342644032921812e-2) * t9636;
    let t9638 = t626 * t705;
    let t9639 = t701 * t9638;
    let t9651 = F::new(1.0) / t191 / t2360;
    (t9609, t9636, t9637, t9639, t9651)
}
