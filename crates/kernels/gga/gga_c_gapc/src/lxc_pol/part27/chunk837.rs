//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 837/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk837<F: Float>(t9680: F, t9683: F, t9686: F, t9690: F, t9693: F, t9698: F, t9704: F, t9707: F, t9711: F, t9714: F, t9717: F, t9719: F, t9724: F) -> F {
    let t9726 = -F::new(0.36954560225358884233e-5) * t9680 + F::new(0.7588373973867992891e-7) * t9683 - F::new(0.13492128925537291361e-6) * t9686 - F::new(0.25745714186718600948e-5) * t9690 + F::new(0.2318836277704281739e-4) * t9693 - F::new(0.37545833188964626383e-6) * t9698 - F::new(0.33199136135672468897e-7) * t9704 + F::new(0.59028064049225649701e-7) * t9707 - F::new(0.93789165502563894766e-9) * t9711 + F::new(0.12647289956446654818e-8) * t9714 + F::new(0.50602213541666666669e-5) * t9717 + F::new(0.13900948042322754167e-2) * t9719 + F::new(0.84410248952307505288e-7) * t9724;
    t9726
}
