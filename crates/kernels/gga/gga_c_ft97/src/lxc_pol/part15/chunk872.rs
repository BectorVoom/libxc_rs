//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 872/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk872<F: Float>(t2101: F, t2179: F, t157: F, t40436: F, t604: F, t7763: F, t143: F, t38052: F, t161: F, t38061: F, t89: F, t40424: F) -> (F, F, F, F, F, F) {
    let t40911 = t2101 * t2179;
    let t40926 = t40436 * t157;
    let t40931 = t604 * t7763;
    let t41002 = t38052 * t143;
    let t41093 = F::new(280.0) / F::new(243.0) * t89 * t38061 * t161;
    let t41251 = t40424 * t157;
    (t40911, t40926, t40931, t41002, t41093, t41251)
}
