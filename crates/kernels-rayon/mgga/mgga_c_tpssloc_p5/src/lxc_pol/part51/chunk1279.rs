//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1279/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1279(t22751: f64, t31145: f64, t22724: f64, t31104: f64, t1377: f64, t6992: f64, t31100: f64, t81228: f64, t81326: f64, t31109: f64, t6883: f64, t31124: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114216 = t22751 * t31145;
    let t114225 = 0.52089578783527170489e-1_f64 * t22724 * t31104;
    let t114226 = t1377 * t6992;
    let t114240 = t81228 * t81326 * t31100;
    let t114242 = t6883 * t31109;
    let t114253 = t6883 * t31124;
    (t114216, t114225, t114226, t114240, t114242, t114253)
}
