//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1189/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1189(t26544: f64, t8531: f64, t113: f64, t9149: f64, t2588: f64, t26530: f64, t157: f64, t62: f64, t9161: f64, t7624: f64, t8755: f64, t36222: f64, t808: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91828 = t8531 * t26544;
    let t91830 = t9149 * t113;
    let t91832 = t2588 * t26530;
    let t91835 = t157 * t62 * t9161;
    let t91837 = t8755 * t7624;
    let t91839 = t808 * t36222;
    (t91828, t91830, t91832, t91835, t91837, t91839)
}
