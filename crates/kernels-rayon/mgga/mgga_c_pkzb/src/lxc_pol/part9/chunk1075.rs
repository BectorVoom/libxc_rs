//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1075/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1075(t127: f64, t16942: f64, t79: f64, t1613: f64, t16588: f64, t541: f64, t555: f64, t1692: f64, t568: f64, t5252: f64, t6892: f64, t146: f64, t1540: f64, t155: f64, t52: f64, t95: f64) -> (f64, f64, f64, f64, f64) {
    let t16946 = 840.0_f64 * t79 / t16942 * t127;
    let t16950 = 0.35089341735807877242e1_f64 * t555 * t1613 * t16588 * t541;
    let t17000 = t568 * t1692;
    let t17009 = t6892 * t5252;
    let t17026 = 455.0_f64 / 243.0_f64 * t146 / t52 / t1540 * t95 * t155;
    (t16946, t16950, t17000, t17009, t17026)
}
