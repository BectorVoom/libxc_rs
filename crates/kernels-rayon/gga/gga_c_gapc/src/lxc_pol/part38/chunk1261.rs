//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1261/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1261(t1404: f64, t3634: f64, t997: f64, t1510: f64, t14541: f64, t1458: f64, t1649: f64, t474: f64, t11199: f64, t8419: f64, t11189: f64, t8524: f64) -> (f64, f64, f64, f64, f64) {
    let t35588 = t997 * t3634 * t1404;
    let t35591 = t997 * t3634 * t1510;
    let t35595 = t14541 * t1458 * t474 * t1649;
    let t35597 = t8419 * t11199;
    let t35599 = t8524 * t11189;
    (t35588, t35591, t35595, t35597, t35599)
}
