//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1226/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1226(t1799: f64, t4341: f64, t19604: f64, t5909: f64, t1811: f64, t198: f64, t205: f64) -> (f64, f64, f64) {
    let t20396 = t4341 * t1799;
    let t20407 = t5909 * t19604;
    let t20417 = t198 * t205 * t1811;
    (t20396, t20407, t20417)
}
