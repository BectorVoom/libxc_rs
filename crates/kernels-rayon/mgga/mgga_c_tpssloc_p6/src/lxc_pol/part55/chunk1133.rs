//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1133/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1133(t225: f64, t22643: f64, t1887: f64, t23069: f64, t229: f64, t268: f64, t6559: f64, t23228: f64, t794: f64, t852: f64, t213: f64, t221: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81326 = t22643 * t225;
    let t81591 = t23069 * t1887;
    let t81651 = t6559 * t229 * t268;
    let t82074 = t23228 * t225;
    let t82133 = t794 * t852;
    let t82159 = t213 * t852 * t225;
    let t82631 = t221 * t697;
    (t81326, t81591, t81651, t82074, t82133, t82159, t82631)
}
