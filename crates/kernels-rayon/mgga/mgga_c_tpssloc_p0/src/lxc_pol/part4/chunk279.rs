//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 279/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk279(t882: f64, t884: f64, t123: f64, t881: f64, t291: f64, t287: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t885 = t882 * t884;
    let t886 = t123 * t885;
    let t888 = -t881 - 0.17808333333333333333e-1_f64 * t886;
    let t890 = 0.621814e-1_f64 * t888 * t291;
    let t891 = t287 * t287;
    let t892 = 1.0_f64 / t891;
    (t885, t886, t888, t890, t891, t892)
}
