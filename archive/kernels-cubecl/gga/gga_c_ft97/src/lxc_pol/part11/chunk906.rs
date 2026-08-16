//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 906/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk906<F: Float>(t1775: F, t8322: F, t8311: F, t8319: F, t8328: F, t8295: F, t8292: F, t11761: F, t11762: F, t1787: F, t3127: F, t3134: F, t37259: F, t37264: F, t37269: F, t37279: F, t37287: F, t37324: F, t38277: F, t38526: F, t462: F, t8291: F, t8327: F) -> F {
    let t38598 = t1775 * t8322;
    let t38600 = t1775 * t8311;
    let t38602 = t1775 * t8319;
    let t38604 = t1775 * t8328;
    let t38606 = t1775 * t8295;
    let t38614 = t1775 * t8292;
    let t38631 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t462 * t1787 * t37279 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t38598 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t38600 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t38602 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t38604 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t38606 + F::cast_from(8.0_f64) * t462 * t1787 * t37287 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t462 * t8327 * t37324 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t38614 - F::cast_from(4.0_f64) * t462 * t8291 * t38277 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t462 * t1787 * t37259 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t462 * t3134 * t37264 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t462 * t3127 * t37269 - F::cast_from(8.0_f64) * t11761 * t11762 * t38526;
    t38631
}
