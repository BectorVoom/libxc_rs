//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 920/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk920<F: Float>(t8668: F, t8671: F, t8678: F, t8682: F, t8688: F, t8691: F, t8694: F, t8698: F, t8702: F, t8705: F, t8707: F, t8717: F, t8720: F) -> F {
    let t10574 = -F::cast_from(0.78385901460875530441e-2_f64) * t8668 - F::cast_from(0.4048307291666666667e-4_f64) * t8671 - F::cast_from(0.59037814670138888894e-5_f64) * t8678 - F::cast_from(0.59037814670138888894e-5_f64) * t8682 + F::cast_from(0.42233783114695867695e-6_f64) * t8688 - F::cast_from(0.2318836277704281739e-4_f64) * t8691 - F::cast_from(0.27801896084645508334e-2_f64) * t8694 + F::cast_from(0.55603792169291016668e-2_f64) * t8698 + F::cast_from(0.12974218172834570556e-1_f64) * t8702 - F::cast_from(0.57970906942607043472e-5_f64) * t8705 + F::cast_from(0.57970906942607043472e-5_f64) * t8707 - F::cast_from(0.71809639497914566863e-8_f64) * t8717 + F::cast_from(0.1349435763888888889e-4_f64) * t8720;
    t10574
}
