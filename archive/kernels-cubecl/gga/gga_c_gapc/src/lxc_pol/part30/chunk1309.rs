//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1309/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1309<F: Float>(t34169: F, t34171: F, t34174: F, t34176: F, t34178: F, t34181: F, t34184: F, t34188: F, t34191: F, t34193: F, t34200: F, t34205: F, t34207: F, t34209: F, t34211: F, t34214: F, t34217: F, t34219: F, t34222: F, t34224: F, t34227: F, t34230: F) -> (F, F) {
    let t38026 = F::cast_from(0.42270452978984302532e-6_f64) * t34169 + F::cast_from(0.13505639832369200846e-5_f64) * t34171 + F::cast_from(0.27011279664738401692e-5_f64) * t34174 + F::cast_from(0.7246363367825880434e-6_f64) * t34176 + F::cast_from(0.14492726735651760868e-5_f64) * t34178 - F::cast_from(0.9275345110817126956e-4_f64) * t34181 + F::cast_from(0.86880925264517213544e-4_f64) * t34184 + F::cast_from(0.68759642991278900876e-8_f64) * t34188 - F::cast_from(0.5691280480400994668e-7_f64) * t34191 + F::cast_from(0.14758978949652777779e-5_f64) * t34193 - F::cast_from(0.77055513242940134824e-7_f64) * t34200;
    let t38039 = F::cast_from(0.8096354166666666667e-4_f64) * t34205 + F::cast_from(0.40481770833333333336e-3_f64) * t34207 + F::cast_from(0.6487109086417285278e-2_f64) * t34209 + F::cast_from(0.49163213094075520838e-7_f64) * t34211 + F::cast_from(0.43440462632258606772e-4_f64) * t34214 - F::cast_from(0.44197102999375800016e-7_f64) * t34217 - F::cast_from(0.31432979653156068972e-7_f64) * t34219 - F::cast_from(0.19336232562226912507e-7_f64) * t34222 - F::cast_from(0.27011279664738401692e-5_f64) * t34224 + F::cast_from(0.1686740451388888889e-5_f64) * t34227 - F::cast_from(0.14758978949652777779e-5_f64) * t34230;
    (t38026, t38039)
}
