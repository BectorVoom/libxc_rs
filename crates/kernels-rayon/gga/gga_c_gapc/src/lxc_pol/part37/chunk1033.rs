//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1033/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1033(t11897: f64, t3322: f64, t11808: f64, t3330: f64, t11302: f64, t7259: f64, t8142: f64, t2660: f64) -> (f64, f64, f64, f64, f64) {
    let t11898 = t11897 * t3322;
    let t11900 = t11808 * t3330;
    let t11902 = t7259 * t11302;
    let t11903 = t11902 * t8142;
    let t11905 = t2660 * t11302;
    (t11898, t11900, t11902, t11903, t11905)
}
