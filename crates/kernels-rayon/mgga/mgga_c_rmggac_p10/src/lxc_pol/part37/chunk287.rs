//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 287/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk287(t2231: f64, t82: f64, t302: f64, t702: f64, t290: f64, t128: f64, t618: f64, t118: f64, t2024: f64, t570: f64, t551: f64, t645: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2232 = t82 * t2231;
    let t2244 = t302 * t702;
    let t2265 = t290 * t702;
    let t2281 = t128 * t618;
    let t2282 = t118 * t2281;
    let t2292 = t2024 * t570;
    let t2295 = t645 * t551;
    (t2232, t2244, t2265, t2281, t2282, t2292, t2295)
}
