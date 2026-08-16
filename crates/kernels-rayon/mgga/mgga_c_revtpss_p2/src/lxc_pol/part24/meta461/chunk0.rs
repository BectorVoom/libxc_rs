//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1432/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1432(t11243: f64, t1802: f64, t1244: f64, t13036: f64, t225: f64, t56331: f64, t480: f64, t1235: f64, t1789: f64, t2434: f64, t371: f64, t12987: f64, t1803: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57403 = t1802 * t11243;
    let t57405 = t13036 * t1244 * t57403;
    let t57465 = t56331 * t225;
    let t57466 = t57465 * t480;
    let t57471 = t1235 * t371 * t2434 * t1789;
    let t57473 = t12987 * t1803;
    (t57403, t57405, t57465, t57466, t57471, t57473)
}
