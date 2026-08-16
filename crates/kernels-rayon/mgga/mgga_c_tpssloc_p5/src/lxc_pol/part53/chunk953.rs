//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 953/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk953(t22716: f64, t8612: f64, t22674: f64, t31607: f64, t6897: f64, t31550: f64, t81228: f64, t81326: f64, t31551: f64, t81159: f64, t115352: f64, t6907: f64) -> (f64, f64, f64, f64, f64) {
    let t115566 = t22716 * t8612;
    let t115572 = t6897 * t22674 * t31607;
    let t115586 = t81228 * t81326 * t31550;
    let t115596 = t81159 * t31551;
    let t115601 = t6897 * t115352 * t6907;
    (t115566, t115572, t115586, t115596, t115601)
}
