//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 948/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk948(t9680: f64, t9683: f64, t9686: f64, t9690: f64, t9693: f64, t9698: f64, t9704: f64, t9707: f64, t9711: f64, t9714: f64, t9717: f64, t9719: f64, t9724: f64) -> f64 {
    let t10915 = -0.73909120450717768468e-5_f64 * t9680 + 0.15176747947735985782e-6_f64 * t9683 - 0.2698425785107458272e-6_f64 * t9686 - 0.51491428373437201896e-5_f64 * t9690 + 0.4637672555408563478e-4_f64 * t9693 - 0.75091666377929252765e-6_f64 * t9698 - 0.66398272271344937795e-7_f64 * t9704 + 0.1180561280984512994e-6_f64 * t9707 - 0.18757833100512778952e-8_f64 * t9711 + 0.25294579912893309636e-8_f64 * t9714 + 0.10120442708333333334e-4_f64 * t9717 + 0.27801896084645508334e-2_f64 * t9719 + 0.16882049790461501058e-6_f64 * t9724;
    t10915
}
