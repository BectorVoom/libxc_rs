//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1185/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1185(t2268: f64, t2304: f64, t31936: f64, t10242: f64, t1595: f64, t1063: f64, t21042: f64, t2765: f64, t25955: f64, t894: f64, t20013: f64, t2854: f64, t6320: f64) -> (f64, f64, f64, f64, f64) {
    let t31939 = 0.39837009289946609438e0_f64 * t2268 * t2304 * t31936;
    let t31942 = 0.28455006635676149599e-1_f64 * t2268 * t1595 * t10242;
    let t31945 = 0.85365019907028448797e-1_f64 * t1063 * t2765 * t21042;
    let t31948 = 0.28455006635676149599e-1_f64 * t1063 * t894 * t25955;
    let t31952 = 0.17073003981405689759e0_f64 * t2268 * t6320 * t2854 * t20013;
    (t31939, t31942, t31945, t31948, t31952)
}
