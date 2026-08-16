//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 376/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk376(t293: f64, t711: f64, t291: f64, t1233: f64, t1236: f64, t1685: f64, t1227: f64, t286: f64, t458: f64, t714: f64, t1237: f64, t1681: f64, t1687: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t1691 = 1.0_f64 / t711 / t293;
    let t1692 = t291 * t1691;
    let t1693 = t1692 * t1233;
    let t1695 = t1236 * t1685 * pi;
    let t1699 = t1227 * t286 * t458;
    let t1700 = t714 * t1699;
    let t1702 = 63.0_f64 / 256.0_f64 * t1681 - 49.0_f64 / 8192.0_f64 * t1237 * t1687 + 49.0_f64 / 24576.0_f64 * t1693 * t1695 - 21.0_f64 / 256.0_f64 * t1700;
    (t1691, t1692, t1695, t1699, t1700, t1702)
}
