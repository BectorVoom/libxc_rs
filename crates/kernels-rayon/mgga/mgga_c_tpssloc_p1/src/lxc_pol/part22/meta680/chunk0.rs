//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2243/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2243(t14080: f64, t4571: f64, t14202: f64, t4644: f64, t10413: f64, t10422: f64, t17700: f64, t1036: f64, t17878: f64, t13969: f64, t17631: f64, t3039: f64) -> (f64, f64, f64, f64, f64) {
    let t62282 = t14080 * t4571;
    let t62284 = t4644 * t14202;
    let t62306 = t10413 * t10422 * t17700;
    let t62343 = t17878 * t1036;
    let t62349 = t3039 * t13969 * t17631;
    (t62282, t62284, t62306, t62343, t62349)
}
