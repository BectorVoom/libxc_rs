//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 291/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk291(t552: f64, t938: f64, t551: f64, t910: f64, t921: f64, t595: f64, t897: f64, t602: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t939 = t552 * t938;
    let t940 = t551 * t939;
    let t943 = t552 * t910;
    let t944 = t551 * t943;
    let t948 = t551 * t552 * t921;
    let t951 = t595 * t897;
    let t955 = 12.0_f64 * t602 + 12.0_f64 * t605;
    (t939, t940, t943, t944, t948, t951, t955)
}
