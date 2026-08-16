//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1052/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1052(t1338: f64, t147: f64, t19: f64, t3156: f64, t1403: f64, t3116: f64, t1457: f64, t632: f64, t1266: f64, t4048: f64, t424: f64, t116: f64, t14873: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25202 = t3156 * t1338 * t19 * t147;
    let t25382 = t3116 * t1403 * t19 * t147;
    let t25514 = t632 * t1457;
    let t25526 = t1266 * t1457;
    let t25530 = t424 * t4048;
    let t25708 = t116 * t14873;
    (t25202, t25382, t25514, t25526, t25530, t25708)
}
