//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1024/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1024(t11068: f64, t590: f64, t1991: f64, t1628: f64, t3495: f64, t1589: f64, t3451: f64, t3464: f64, t769: f64, t10667: f64, t314: f64, t313: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11069 = t11068 * t590;
    let t11071 = 0.1022478025437886658e1_f64 * t1991 * t11069;
    let t11072 = t1628 * t3495;
    let t11075 = t1589 * t3451;
    let t11080 = t769 * t3464;
    let t11083 = t314 * t10667;
    let t11084 = t313 * t11083;
    (t11069, t11071, t11072, t11075, t11080, t11083, t11084)
}
