//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 715/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk715(t1397: f64, t2371: f64, t1: f64, t6540: f64, t544: f64, t1402: f64, t2339: f64, t447: f64, t6509: f64, t204: f64, t1433: f64, t2486: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6696 = t1397 * t2371;
    let t6699 = t6540 * t1;
    let t6700 = t544 * t6699;
    let t6703 = t1402 * t2339;
    let t6706 = t6509 * t447;
    let t6707 = t204 * t6706;
    let t6710 = t1433 * t2486;
    (t6696, t6699, t6700, t6703, t6706, t6707, t6710)
}
