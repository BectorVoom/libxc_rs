//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 464/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk464(t5726: f64, t913: f64, t893: f64, t2844: f64, t5694: f64, t2842: f64, t2848: f64, t4335: f64, t5679: f64, t5683: f64, t5687: f64, t1568: f64) -> (f64, f64, f64, f64) {
    let t5727 = t5726 * t913;
    let t5729 = 1.0_f64 * t893 * t5727;
    let t5730 = t5694 * t2844;
    let t5732 = 0.16081979498692535067e2_f64 * t2842 * t5730;
    let t5737 = t2848 + 0.11415555555555555555e-1_f64 * t4335 - 0.11415555555555555555e-1_f64 * t5679 + 0.34246666666666666666e-1_f64 * t5683 - 0.17123333333333333333e-1_f64 * t5687;
    let t5742 = t1568 * t1568;
    (t5729, t5732, t5737, t5742)
}
