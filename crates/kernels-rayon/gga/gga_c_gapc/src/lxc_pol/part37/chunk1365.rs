//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1365/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1365(t33150: f64, t33154: f64, t33156: f64, t33160: f64, t33162: f64, t33165: f64, t33167: f64, t33170: f64, t33173: f64, t33175: f64, t36506: f64, t33209: f64) -> (f64, f64) {
    let t36507 = -0.21135226489492151266e-6_f64 * t33150 + 0.80189736504692130024e-6_f64 * t33154 + 0.63307686714230628966e-7_f64 * t33156 - 0.99041358770707472873e-5_f64 * t33160 - 0.13259130899812740005e-6_f64 * t33162 - 0.44197102999375800018e-8_f64 * t33165 - 0.66295654499063700026e-7_f64 * t33167 + 0.43440462632258606772e-4_f64 * t33170 + 0.11372686522837130914e-5_f64 * t33173 + 0.10298285674687440379e-4_f64 * t33175 + t36506;
    let t36508 = 0.13505639832369200846e-5_f64 * t33209;
    (t36507, t36508)
}
