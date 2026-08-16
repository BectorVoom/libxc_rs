//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 347/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk347(t906: f64, t1380: f64, t1383: f64, t1385: f64, t1387: f64, t1389: f64, t1391: f64, t1392: f64, t764: f64, t774: f64, t782: f64, t905: f64, t914: f64) -> f64 {
    let t1393 = 4.0_f64 * t906;
    let t1394 = t1380 - t1383 + t1385 - t1387 + t1389 + t1391 + t914 - t1392 - t905 - t1393 - t764 + t774 + t782;
    t1394
}
