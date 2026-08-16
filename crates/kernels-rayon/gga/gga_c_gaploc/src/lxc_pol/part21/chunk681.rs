//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 681/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk681(t1266: f64, t2304: f64, t1624: f64, t876: f64, t2295: f64, t535: f64, t2440: f64, t448: f64, t1306: f64, t894: f64, t1227: f64, t130: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6342 = t2304 * t1266;
    let t6345 = t1624 * t876;
    let t6348 = t535 * t2295;
    let t6353 = t2440 * t448;
    let t6356 = t894 * t1306;
    let t6361 = t130 * t1227;
    (t6342, t6345, t6348, t6353, t6356, t6361)
}
