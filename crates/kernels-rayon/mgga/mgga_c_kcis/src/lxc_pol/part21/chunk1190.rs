//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1190/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1190(t2150: f64, t9175: f64, t2526: f64, t755: f64, t7627: f64, t26553: f64, t774: f64, t8537: f64, t8538: f64, t153: f64, t822: f64, t2484: f64, t26547: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91841 = t9175 * t2150;
    let t91844 = t755 * t7627 * t2526;
    let t91847 = t755 * t26553 * t774;
    let t91850 = t8537 * t2150 * t8538;
    let t91852 = t153 * t822;
    let t91854 = t2484 * t26547;
    (t91841, t91844, t91847, t91850, t91852, t91854)
}
