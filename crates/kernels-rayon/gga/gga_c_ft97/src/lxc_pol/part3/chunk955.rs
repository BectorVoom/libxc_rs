//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 955/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk955(t18396: f64, t18429: f64, t18454: f64, t18511: f64, t18599: f64, t18639: f64, t18706: f64, t18754: f64, t1137: f64, t1173: f64, t17713: f64, t17715: f64, t17718: f64, t18178: f64, t247: f64, t263: f64, t3683: f64, t3827: f64, t4003: f64, t4915: f64, t5059: f64, t5179: f64, t719: f64, t771: f64) -> f64 {
    let t18757 = t18396 + t18429 + t18454 + t18511 + t18599 + t18639 + t18706 + t18754;
    let t18759 = -2.0_f64 * t1137 * t4003 - 2.0_f64 * t1173 * t3683 - 2.0_f64 * t1173 * t3827 - t17713 * t263 - t17715 * t263 - t17718 * t263 - t18178 * t263 - t18757 * t247 - t4915 * t771 - t5059 * t771 - t5179 * t719;
    t18759
}
