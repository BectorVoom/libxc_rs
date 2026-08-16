//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1125/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1125<F: Float>(t43537: F, t2789: F, t2755: F, t91: F, t190: F, t2680: F, t305: F, t36452: F, t37991: F, t2756: F, t824: F, t9853: F) -> (F, F, F, F) {
    let t43538 = F::cast_from(280.0_f64) / F::cast_from(243.0_f64) * t43537;
    let t43539 = t2789 * t2789;
    let t43541 = t91 * t2755 * t43539;
    let t43548 = F::cast_from(1.0_f64) / t305 / t37991 / t190 / t2680 / t36452 / F::cast_from(96.0_f64);
    let t43549 = t2756 * t2756;
    let t43551 = t91 * t43548 * t43549;
    let t43553 = t9853 * t824;
    (t43538, t43541, t43551, t43553)
}
