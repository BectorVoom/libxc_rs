//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 745/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk745(t2489: f64, t7014: f64, t1366: f64, t2465: f64, t2464: f64, t587: f64, t4167: f64, t2487: f64, t1415: f64, t1428: f64) -> (f64, f64, f64, f64) {
    let t7015 = t7014 * t2489;
    let t7017 = t2465 * t1366;
    let t7018 = t2464 * t7017;
    let t7019 = t587 * t7018;
    let t7021 = t2465 * t4167;
    let t7022 = t2464 * t7021;
    let t7023 = t2487 * t7022;
    let t7025 = t1415 * t1428;
    (t7015, t7019, t7023, t7025)
}
