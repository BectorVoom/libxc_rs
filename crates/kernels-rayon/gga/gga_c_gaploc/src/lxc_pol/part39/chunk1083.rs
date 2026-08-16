//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1083/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1083(t105: f64, t169: f64, t172: f64, t452: f64, t46952: f64, t42756: f64, t42759: f64, t42763: f64, t42767: f64, t42771: f64, t42772: f64, t42773: f64, t42774: f64, t42778: f64, t42782: f64) -> f64 {
    let t46991 = 0.28455006635676149599e-1_f64 * t105 * t452 * t46952 * t169 * t172;
    let t46996 = -t42756 + t46991 + 0.28455006635676149599e-1_f64 * t42759 + t42763 + t42767 - t42771 - t42772 + t42773 - 0.15808337019820083111e-2_f64 * t42774 - 0.19918504644973304719e0_f64 * t42778 + 0.34146007962811379518e0_f64 * t42782;
    t46996
}
