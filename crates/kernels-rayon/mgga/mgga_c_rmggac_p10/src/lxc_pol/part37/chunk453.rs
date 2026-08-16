//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 453/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk453(t1664: f64, t702: f64, t1614: f64, t699: f64, t2471: f64, t333: f64, t321: f64, t570: f64, t8264: f64, t2228: f64, t551: f64, t8710: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9343 = t1664 * t702;
    let t9352 = t699 * t1614;
    let t9370 = t2471 * t333;
    let t9383 = t2471 * t321;
    let t9427 = t8264 * t570;
    let t9437 = t2228 * t551;
    let t9445 = 0.4838420607177634088e-3_f64 * t8710;
    (t9343, t9352, t9370, t9383, t9427, t9437, t9445)
}
