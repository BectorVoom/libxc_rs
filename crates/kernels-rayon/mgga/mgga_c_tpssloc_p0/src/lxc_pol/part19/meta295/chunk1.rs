//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1077/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1077(t13004: f64, t205: f64, t4126: f64, t782: f64, t68: f64, t822: f64, t2644: f64, t820: f64, t2617: f64, t4177: f64, t2628: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13005 = t205 * t13004;
    let t13012 = t782 * t4126;
    let t13151 = t822 * t68;
    let t13222 = t2644 * t820;
    let t13254 = t2617 * t4177;
    let t13257 = t2628 * t836;
    (t13005, t13012, t13151, t13222, t13254, t13257)
}
