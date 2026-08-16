//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 472/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk472(t203: f64, t883: f64, t900: f64, t1359: f64, t874: f64, t1397: f64, t2371: f64, t1: f64, t6540: f64, t544: f64, t1433: f64, t2486: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6589 = t883 * t203;
    let t6590 = t900 * t6589;
    let t6603 = t1359 * t874;
    let t6696 = t1397 * t2371;
    let t6699 = t6540 * t1;
    let t6700 = t544 * t6699;
    let t6710 = t1433 * t2486;
    (t6589, t6590, t6603, t6696, t6700, t6710)
}
