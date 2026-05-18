//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 876/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk876<F: Float>(t16710: F, t16751: F, t173: F, t184: F, t199: F, t1673: F, t1680: F, t5373: F, t636: F, t422: F, t661: F, t1416: F) -> (F, F, F, F) {
    let t16756 = F::new(2.0) / F::new(15.0) * t173 * (t16710 + t16751) * t184 * t199;
    let t16757 = t1680 * t1673;
    let t16758 = F::new(16.0) / F::new(45.0) * t16757;
    let t16759 = t5373 * t636;
    let t16760 = F::new(16.0) / F::new(45.0) * t16759;
    let t16761 = t422 * t661;
    let t16762 = t16761 * t1416;
    (t16756, t16758, t16760, t16762)
}
