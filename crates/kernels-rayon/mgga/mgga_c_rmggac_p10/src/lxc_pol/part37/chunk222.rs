//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 222/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk222(t321: f64, t552: f64, t333: f64, t529: f64, t941: f64, t537: f64, t809: f64, t312: f64, t50: f64, t90: f64, t814: f64, t547: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1551 = t552 * t321;
    let t1554 = t552 * t333;
    let t1562 = t941 * t529;
    let t1569 = t809 * t537;
    let t1570 = t1569 * t312;
    let t1573 = t90 * t50;
    let t1574 = t1573 * t814;
    let t1579 = t820 * t547;
    (t1551, t1554, t1562, t1570, t1574, t1579)
}
