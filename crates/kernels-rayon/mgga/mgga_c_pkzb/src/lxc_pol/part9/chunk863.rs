//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 863/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk863(t6262: f64, t871: f64, t2295: f64, t877: f64, t6122: f64, t890: f64, t2256: f64, t858: f64, t2258: f64, t870: f64) -> (f64, f64, f64, f64, f64) {
    let t6263 = t6262 * t871;
    let t6266 = t877 * t2295;
    let t6269 = t6122 * t890;
    let t6272 = t858 * t2256;
    let t6275 = t2258 * t870;
    (t6263, t6266, t6269, t6272, t6275)
}
