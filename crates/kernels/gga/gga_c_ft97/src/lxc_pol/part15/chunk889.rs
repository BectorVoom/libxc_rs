//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 889/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk889<F: Float>(t2360: F, t2842: F, t309: F, t43917: F, t192: F, t33828: F, t43833: F, t870: F, t9570: F, t313: F, t41743: F, t89: F) -> (F, F, F, F, F, F) {
    let t44204 = t2842 * t2360;
    let t44245 = t43917 * t309;
    let t44280 = t192 * t33828;
    let t44335 = t43833 * t309;
    let t44340 = t870 * t9570;
    let t44436 = F::cast_from(280.0_f64) / F::cast_from(243.0_f64) * t89 * t41743 * t313;
    (t44204, t44245, t44280, t44335, t44340, t44436)
}
