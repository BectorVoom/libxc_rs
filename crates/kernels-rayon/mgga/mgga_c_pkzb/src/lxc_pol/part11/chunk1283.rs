//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1283/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1283(t1167: f64, t218: f64, t219: f64, t9795: f64, t11153: f64, t824: f64, t31086: f64, t334: f64, t11205: f64, t675: f64, t11209: f64, t3747: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31254 = t218 * t219 * t1167 * t9795;
    let t31258 = t218 * t219 * t824 * t11153;
    let t31262 = t218 * t219 * t334 * t31086;
    let t31265 = t218 * t675 * t11205;
    let t31268 = t218 * t675 * t11209;
    let t31270 = t3747 * t836;
    (t31254, t31258, t31262, t31265, t31268, t31270)
}
