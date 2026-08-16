//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 734/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk734(t1932: f64, t2972: f64, t2975: f64, t8622: f64, t8626: f64, t8629: f64, t8632: f64, t8634: f64, t8637: f64, t8641: f64, t8645: f64, t8647: f64, t8650: f64, t8657: f64, t8660: f64) -> f64 {
    let t8662 = t1932 * t2972;
    let t8663 = t8662 * t2975;
    let t8665 = 0.16413631885237615283e-7_f64 * t8622 - 0.57970906942607043472e-5_f64 * t8626 - 0.27801896084645508334e-2_f64 * t8629 - 0.27801896084645508334e-2_f64 * t8632 - 0.6956508833112845217e-4_f64 * t8634 - 0.11255061864162936194e-6_f64 * t8637 - 0.23248749138441366393e-5_f64 * t8641 - 0.17376185052903442709e-3_f64 * t8645 - 0.12163329537032409896e-2_f64 * t8647 + 0.21135226489492151266e-6_f64 * t8650 + 0.61644410594352107859e-7_f64 * t8657 - 0.27801896084645508334e-2_f64 * t8660 + 0.12163329537032409896e-2_f64 * t8663;
    t8665
}
