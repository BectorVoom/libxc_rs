//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2014/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2014(t22734: f64, t81159: f64, t22899: f64, t6914: f64, t22715: f64, t6887: f64, t6970: f64, t22751: f64, t22883: f64, t12225: f64, t22641: f64, t22690: f64, t6969: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81160 = t81159 * t22734;
    let t81184 = t6914 * t22899;
    let t81186 = t22715 * t6887;
    let t81187 = t81186 * t6970;
    let t81189 = t22751 * t22883;
    let t81195 = t22641 * t12225;
    let t81197 = t81195 * t22690 * t6969;
    (t81160, t81184, t81186, t81187, t81189, t81195, t81197)
}
