//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 854/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk854<F: Float>(t9680: F, t9683: F, t9686: F, t9690: F, t9693: F, t9698: F, t9704: F, t9707: F, t9711: F, t9714: F, t9717: F, t9719: F, t9724: F) -> F {
    let t9726 = -F::cast_from(0.36954560225358884233e-5_f64) * t9680 + F::cast_from(0.7588373973867992891e-7_f64) * t9683 - F::cast_from(0.13492128925537291361e-6_f64) * t9686 - F::cast_from(0.25745714186718600948e-5_f64) * t9690 + F::cast_from(0.2318836277704281739e-4_f64) * t9693 - F::cast_from(0.37545833188964626383e-6_f64) * t9698 - F::cast_from(0.33199136135672468897e-7_f64) * t9704 + F::cast_from(0.59028064049225649701e-7_f64) * t9707 - F::cast_from(0.93789165502563894766e-9_f64) * t9711 + F::cast_from(0.12647289956446654818e-8_f64) * t9714 + F::cast_from(0.50602213541666666669e-5_f64) * t9717 + F::cast_from(0.13900948042322754167e-2_f64) * t9719 + F::cast_from(0.84410248952307505288e-7_f64) * t9724;
    t9726
}
