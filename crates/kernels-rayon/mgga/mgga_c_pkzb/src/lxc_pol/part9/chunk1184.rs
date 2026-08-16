//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1184/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1184(t1987: f64, t7568: f64, t237: f64, t5838: f64, t1971: f64, t721: f64, t2852: f64, t2149: f64, t803: f64, t7555: f64, t2860: f64, t5809: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20636 = 0.17544670867903938621e1_f64 * t1987 * t7568;
    let t20637 = t237 * t5838;
    let t20638 = t1971 * t721;
    let t20641 = 0.31168546390226634765e3_f64 * t20637 * t2852 * t20638;
    let t20642 = t2149 * t803;
    let t20647 = 0.35089341735807877242e1_f64 * t1987 * t7555;
    let t20649 = 0.35089341735807877242e1_f64 * t2860 * t5809;
    (t20636, t20638, t20641, t20642, t20647, t20649)
}
