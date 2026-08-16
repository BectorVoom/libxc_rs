//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 262/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk262(t1241: f64, t1251: f64, t1191: f64, t1236: f64, t1238: f64, t498: f64, t500: f64) -> (f64, f64, f64) {
    let t1252 = t1241 * t1251;
    let t1254 = t1191 * t498 + t1236 * t498 - t1238 * t1252;
    let t1256 = 1.0_f64 / t500;
    (t1252, t1254, t1256)
}
