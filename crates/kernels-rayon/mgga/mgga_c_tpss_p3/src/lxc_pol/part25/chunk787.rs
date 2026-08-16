//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 787/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk787(t1259: f64, t5448: f64, t1256: f64, t1657: f64, t4490: f64, t538: f64, t5428: f64, t5433: f64) -> (f64, f64) {
    let t5449 = t1259 * t5448;
    let t5451 = 2.0_f64 * t1256 * t5433 - t1256 * t5449 - 2.0_f64 * t1657 * t4490 + t538 * t5428;
    (t5449, t5451)
}
