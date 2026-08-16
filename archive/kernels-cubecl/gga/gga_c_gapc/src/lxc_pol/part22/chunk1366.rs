//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1366/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1366<F: Float>(t33150: F, t33154: F, t33156: F, t33160: F, t33162: F, t33165: F, t33167: F, t33170: F, t33173: F, t33175: F, t36506: F, t33209: F) -> (F, F) {
    let t36507 = -F::cast_from(0.21135226489492151266e-6_f64) * t33150 + F::cast_from(0.80189736504692130024e-6_f64) * t33154 + F::cast_from(0.63307686714230628966e-7_f64) * t33156 - F::cast_from(0.99041358770707472873e-5_f64) * t33160 - F::cast_from(0.13259130899812740005e-6_f64) * t33162 - F::cast_from(0.44197102999375800018e-8_f64) * t33165 - F::cast_from(0.66295654499063700026e-7_f64) * t33167 + F::cast_from(0.43440462632258606772e-4_f64) * t33170 + F::cast_from(0.11372686522837130914e-5_f64) * t33173 + F::cast_from(0.10298285674687440379e-4_f64) * t33175 + t36506;
    let t36508 = F::cast_from(0.13505639832369200846e-5_f64) * t33209;
    (t36507, t36508)
}
