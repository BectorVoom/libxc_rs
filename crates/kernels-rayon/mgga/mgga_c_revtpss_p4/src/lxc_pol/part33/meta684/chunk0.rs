//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2251/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2251(t29089: f64, t5357: f64, t21251: f64, t7607: f64, t21254: f64, t104708: f64, t104721: f64, t104888: f64, t104933: f64, t20929: f64, t21210: f64, t29037: f64, t5270: f64, t5348: f64, t5369: f64, t5407: f64, t97174: f64, t97247: f64) -> f64 {
    let t112433 = t29089 * t5357;
    let t112435 = t7607 * t21251;
    let t112437 = t7607 * t21254;
    let t112447 = -0.11433071498151929859e-2_f64 * t29037 * t5270 - 0.95275595817932748827e-4_f64 * t97247 - 0.38110238327173099531e-3_f64 * t104933 + t29089 * t5369 / 54.0_f64 - t7607 * t21210 / 288.0_f64 + t112433 / 162.0_f64 - t112435 / 864.0_f64 - t112437 / 432.0_f64 + 0.57165357490759649296e-3_f64 * t97174 * t20929 + 0.45732285992607719436e-2_f64 * t104708 * t5348 - 0.57165357490759649296e-3_f64 * t104888 * t5407 + 0.30488190661738479624e-2_f64 * t104721 * t5407;
    t112447
}
