//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 715/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk715(t10143: f64, t1914: f64, t134: f64, t221: f64, t3034: f64, t371: f64, t2752: f64, t28: f64, t22468: f64, t2094: f64, t531: f64, t7025: f64, t9239: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23295 = t1914 * t10143;
    let t23383 = t221 * t134;
    let t23508 = 1.0_f64 / t3034 / t371;
    let t23598 = 1.0_f64 / t3034;
    let t23788 = t2752 * t28;
    let t23912 = 22.0_f64 / 9.0_f64 * t22468;
    let t23957 = t531 * t2094;
    let t23963 = t9239 * t7025;
    (t23295, t23383, t23508, t23598, t23788, t23912, t23957, t23963)
}
