//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 932/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk932(t10340: f64, t1445: f64, t1562: f64, t2293: f64, t12919: f64, t4953: f64, t3116: f64, t8097: f64, t10215: f64, t1429: f64, t2365: f64, t2366: f64) -> (f64, f64, f64, f64) {
    let t42015 = t1562 * t1445 * t10340 * t2293;
    let t42018 = 0.69017266717057349418e1_f64 * t4953 * t12919;
    let t42022 = 0.69017266717057349418e1_f64 * t1562 * t1445 * t8097 * t3116;
    let t42026 = t1429 * t2365 * t2366 * t10215;
    (t42015, t42018, t42022, t42026)
}
