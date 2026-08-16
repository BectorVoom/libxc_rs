//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1246/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1246(t1426: f64, t7108: f64, t9031: f64, t997: f64, t9039: f64, t978: f64, t2537: f64, t3524: f64, t1414: f64, t7001: f64, t2598: f64, t3557: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25627 = t1426 * t7108;
    let t25630 = t9031 * t997;
    let t25633 = t9039 * t978;
    let t25643 = t3524 * t2537;
    let t25648 = t1414 * t7001;
    let t25651 = t3557 * t2598;
    (t25627, t25630, t25633, t25643, t25648, t25651)
}
