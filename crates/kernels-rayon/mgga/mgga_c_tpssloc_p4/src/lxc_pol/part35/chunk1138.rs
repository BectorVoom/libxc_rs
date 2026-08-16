//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1138/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1138(t134: f64, t221: f64, t3034: f64, t371: f64, t2752: f64, t28: f64, t2274: f64, t50: f64, t7245: f64, t9239: f64, t2127: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23383 = t221 * t134;
    let t23508 = 1.0_f64 / t3034 / t371;
    let t23598 = 1.0_f64 / t3034;
    let t23788 = t2752 * t28;
    let t24498 = t50 * t2274;
    let t24514 = t9239 * t7245;
    let t24574 = t2127 * t23383;
    (t23383, t23508, t23598, t23788, t24498, t24514, t24574)
}
