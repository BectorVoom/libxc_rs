//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 950/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk950(t1406: f64, t1828: f64, t5218: f64, t5219: f64, t108: f64, t1878: f64, t267: f64, t5221: f64, t17591: f64, t17596: f64, t17601: f64, t17606: f64, t17608: f64, t17610: f64, t17613: f64, t17617: f64, t17621: f64) -> (f64, f64, f64) {
    let t17625 = 32.0_f64 / 15.0_f64 * t5218 * t5219 * t1406 * t1828;
    let t17627 = t1878 * t108 * t267;
    let t17629 = 64.0_f64 / 15.0_f64 * t17627 * t5221;
    let t17630 = t17591 + t17596 - t17601 - t17606 - t17608 + t17610 - t17613 - t17617 + t17621 - t17625 - t17629;
    (t17625, t17629, t17630)
}
