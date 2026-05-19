//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 737/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk737<F: Float>(t1932: F, t2972: F, t2975: F, t8622: F, t8626: F, t8629: F, t8632: F, t8634: F, t8637: F, t8641: F, t8645: F, t8647: F, t8650: F, t8657: F, t8660: F) -> F {
    let t8662 = t1932 * t2972;
    let t8663 = t8662 * t2975;
    let t8665 = F::cast_from(0.16413631885237615283e-7_f64) * t8622 - F::cast_from(0.57970906942607043472e-5_f64) * t8626 - F::cast_from(0.27801896084645508334e-2_f64) * t8629 - F::cast_from(0.27801896084645508334e-2_f64) * t8632 - F::cast_from(0.6956508833112845217e-4_f64) * t8634 - F::cast_from(0.11255061864162936194e-6_f64) * t8637 - F::cast_from(0.23248749138441366393e-5_f64) * t8641 - F::cast_from(0.17376185052903442709e-3_f64) * t8645 - F::cast_from(0.12163329537032409896e-2_f64) * t8647 + F::cast_from(0.21135226489492151266e-6_f64) * t8650 + F::cast_from(0.61644410594352107859e-7_f64) * t8657 - F::cast_from(0.27801896084645508334e-2_f64) * t8660 + F::cast_from(0.12163329537032409896e-2_f64) * t8663;
    t8665
}
