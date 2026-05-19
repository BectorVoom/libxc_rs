//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 946/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk946<F: Float>(t9680: F, t9683: F, t9686: F, t9690: F, t9693: F, t9698: F, t9704: F, t9707: F, t9711: F, t9714: F, t9717: F, t9719: F, t9724: F) -> F {
    let t10915 = -F::cast_from(0.73909120450717768468e-5_f64) * t9680 + F::cast_from(0.15176747947735985782e-6_f64) * t9683 - F::cast_from(0.2698425785107458272e-6_f64) * t9686 - F::cast_from(0.51491428373437201896e-5_f64) * t9690 + F::cast_from(0.4637672555408563478e-4_f64) * t9693 - F::cast_from(0.75091666377929252765e-6_f64) * t9698 - F::cast_from(0.66398272271344937795e-7_f64) * t9704 + F::cast_from(0.1180561280984512994e-6_f64) * t9707 - F::cast_from(0.18757833100512778952e-8_f64) * t9711 + F::cast_from(0.25294579912893309636e-8_f64) * t9714 + F::cast_from(0.10120442708333333334e-4_f64) * t9717 + F::cast_from(0.27801896084645508334e-2_f64) * t9719 + F::cast_from(0.16882049790461501058e-6_f64) * t9724;
    t10915
}
