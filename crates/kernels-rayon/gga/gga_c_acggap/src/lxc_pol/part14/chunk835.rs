//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 835/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk835(t598: f64, t9724: f64, t142: f64, t1866: f64, t7436: f64, t7815: f64, t2030: f64, t1782: f64, t7351: f64, t2060: f64, t7718: f64, t7726: f64, t7738: f64, t7740: f64, t7743: f64, t7748: f64, t7776: f64, t7782: f64, t7788: f64, t7801: f64, t7803: f64, t9335: f64, t9713: f64, t9715: f64, t9717: f64, t9721: f64) -> (f64, f64, f64, f64, f64) {
    let t9725 = t598 * t9724;
    let t9727 = t142 * t1866;
    let t9728 = t7436 * t9727;
    let t9730 = t7815 * t1866;
    let t9731 = t2030 * t9730;
    let t9733 = t7351 * t1782;
    let t9734 = t142 * t9733;
    let t9735 = t2060 * t9734;
    let t9737 = -0.4584375e-1_f64 * t9713 - 0.17149607247227894789e-2_f64 * t9715 + 0.17149607247227894789e-2_f64 * t9717 + 0.31448092289604152068e-2_f64 * t9721 - t7718 - t7726 + t9335 + 0.47172138434406228102e-3_f64 * t9725 - t7738 - t7740 + t7743 + t7748 + t9728 / 24.0_f64 + t9731 / 64.0_f64 - 0.22921875e-1_f64 * t9735 - t7776 + t7782 - t7788 + t7801 - t7803;
    (t9727, t9730, t9733, t9734, t9737)
}
