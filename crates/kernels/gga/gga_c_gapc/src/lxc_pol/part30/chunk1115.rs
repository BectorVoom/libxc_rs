//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1115/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1115<F: Float>(t34104: F, t34108: F, t34111: F, t34114: F, t34117: F, t34119: F, t34121: F, t34125: F, t34127: F, t34132: F, t34135: F, t34169: F, t34171: F, t34174: F, t34176: F, t34178: F, t34181: F, t34184: F, t34188: F, t34191: F, t34193: F, t34200: F) -> (F, F) {
    let t38001 = 0.9275345110817126956e-4 * t34104 + 0.22544241588791628019e-6 * t34108 + 0.13900948042322754167e-2 * t34111 - 0.98326426188151041676e-7 * t34114 + 0.49163213094075520838e-8 * t34117 - 0.14068374825384584215e-7 * t34119 - 0.14068374825384584215e-7 * t34121 + 0.19191204183684243232e-6 * t34125 + 0.68358185972367904025e-5 * t34127 - 0.49163213094075520838e-8 * t34132 + 0.5060221354166666667e-5 * t34135;
    let t38026 = 0.42270452978984302532e-6 * t34169 + 0.13505639832369200846e-5 * t34171 + 0.27011279664738401692e-5 * t34174 + 0.7246363367825880434e-6 * t34176 + 0.14492726735651760868e-5 * t34178 - 0.9275345110817126956e-4 * t34181 + 0.86880925264517213544e-4 * t34184 + 0.68759642991278900876e-8 * t34188 - 0.5691280480400994668e-7 * t34191 + 0.14758978949652777779e-5 * t34193 - 0.77055513242940134824e-7 * t34200;
    (t38001, t38026)
}
