//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1153/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1153(t5373: f64, t6897: f64, t1020: f64, t164: f64, t600: f64, t7084: f64, t5257: f64, t6958: f64, t1034: f64, t5367: f64, t1753: f64, t2639: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20071 = t6897 * t5373;
    let t20075 = t1020 * t5373;
    let t20081 = t7084 * t600 * t164;
    let t20085 = t5257 * t6958;
    let t20093 = t1034 * t5367 * t164;
    let t20102 = t2639 * t1753 * t164;
    (t20071, t20075, t20081, t20085, t20093, t20102)
}
