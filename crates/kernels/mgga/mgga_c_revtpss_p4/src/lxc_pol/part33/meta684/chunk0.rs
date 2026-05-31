//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2251/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2251<F: Float>(t29089: F, t5357: F, t21251: F, t7607: F, t21254: F, t104708: F, t104721: F, t104888: F, t104933: F, t20929: F, t21210: F, t29037: F, t5270: F, t5348: F, t5369: F, t5407: F, t97174: F, t97247: F) -> F {
    let t112433 = t29089 * t5357;
    let t112435 = t7607 * t21251;
    let t112437 = t7607 * t21254;
    let t112447 = -F::cast_from(0.11433071498151929859e-2_f64) * t29037 * t5270 - F::cast_from(0.95275595817932748827e-4_f64) * t97247 - F::cast_from(0.38110238327173099531e-3_f64) * t104933 + t29089 * t5369 / F::cast_from(54.0_f64) - t7607 * t21210 / F::cast_from(288.0_f64) + t112433 / F::cast_from(162.0_f64) - t112435 / F::cast_from(864.0_f64) - t112437 / F::cast_from(432.0_f64) + F::cast_from(0.57165357490759649296e-3_f64) * t97174 * t20929 + F::cast_from(0.45732285992607719436e-2_f64) * t104708 * t5348 - F::cast_from(0.57165357490759649296e-3_f64) * t104888 * t5407 + F::cast_from(0.30488190661738479624e-2_f64) * t104721 * t5407;
    t112447
}
