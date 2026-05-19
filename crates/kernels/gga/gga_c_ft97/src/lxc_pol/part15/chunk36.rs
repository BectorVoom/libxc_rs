//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 36/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk36<F: Float>(t69: F, t72: F, t68: F, t66: F, t64: F) -> (F, F, F, F, F, F) {
    let t73 = t69 * t72;
    let t74 = t68 * t73;
    let t76 = F::new(1.0) + F::cast_from(0.19153082513888888889e-1_f64) * t74;
    let t77 = F::new(1.0) / t76;
    let t78 = t66 * t77;
    let t79 = t64 * t78;
    let t80 = F::new(0.1e-59) < t79;
    let t81 = piecewise3::<F>(t80, t79, F::new(0.1e-59));
    (t74, t76, t77, t78, t81, t79)
}
