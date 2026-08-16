//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 342/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk342(t118: f64, t1302: f64, t122: f64, t429: f64, t1303: f64, t437: f64) -> (f64, f64) {
    let t1501 = t1302 * t118;
    let t1502 = t1501 * t122;
    let t1503 = t429 * t1502;
    let t1504 = t437 * t1303;
    (t1503, t1504)
}
