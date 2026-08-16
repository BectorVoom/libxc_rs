//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1224/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1224(t1976: f64, t2874: f64, t730: f64, t7474: f64, t1987: f64, t7532: f64, t1107: f64, t17474: f64, t17478: f64, t5484: f64, t2860: f64, t5486: f64) -> (f64, f64, f64, f64) {
    let t21299 = 0.51947577317044391277e2_f64 * t730 * t1976 * t7474 * t2874;
    let t21301 = 0.31168546390226634765e3_f64 * t1987 * t7532;
    let t21306 = 0.91082604192152556044e5_f64 * t730 * t17474 * t1107 * t17478 * t5484;
    let t21308 = 0.35089341735807877242e1_f64 * t2860 * t5486;
    (t21299, t21301, t21306, t21308)
}
