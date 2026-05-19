//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 985/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk985<F: Float>(t3851: F, t46261: F, t36166: F, t36168: F, t43615: F, t46232: F, t46235: F, t46238: F, t46242: F, t46244: F, t46246: F, t46248: F, t46250: F, t46252: F, t46254: F, t46256: F, t46259: F) -> F {
    let t46262 = t3851 * t46261;
    let t46264 = F::cast_from(0.5987120850931904282e-1_f64) * t46232 - F::cast_from(0.39828462315181744017e-2_f64) * t46235 + F::cast_from(0.79656924630363488034e-2_f64) * t46238 - t43615 - F::cast_from(0.97567895348519921636e-1_f64) * t36166 + F::cast_from(0.14635184302277988245e0_f64) * t36168 + F::cast_from(0.39828462315181744016e-2_f64) * t46242 - F::cast_from(0.13939961810313610406e-1_f64) * t46244 + F::cast_from(0.22303938896501776649e-1_f64) * t46246 + F::cast_from(0.2993560425465952141e0_f64) * t46248 - F::cast_from(0.11974241701863808564e0_f64) * t46250 + F::cast_from(0.5987120850931904282e-1_f64) * t46252 + F::cast_from(0.5987120850931904282e-1_f64) * t46254 - F::cast_from(0.11974241701863808564e0_f64) * t46256 - F::cast_from(0.5987120850931904282e-1_f64) * t46259 + F::cast_from(0.2993560425465952141e-1_f64) * t46262;
    t46264
}
