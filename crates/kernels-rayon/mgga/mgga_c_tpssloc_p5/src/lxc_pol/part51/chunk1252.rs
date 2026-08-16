//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1252/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1252(t111: f64, t7002: f64, t7222: f64, t193: f64, t201: f64, t7109: f64, t10143: f64, t7758: f64, t112: f64, t26509: f64, t25: f64, t40772: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t83980 = t7002 * t111;
    let t84033 = t7222 * t111;
    let t84797 = t193 * t201 * t7109;
    let t84800 = t7109 * t10143;
    let t86647 = t7758 * t111;
    let t86656 = t26509 * t112;
    let t86716 = t40772 * t25;
    (t83980, t84033, t84797, t84800, t86647, t86656, t86716)
}
