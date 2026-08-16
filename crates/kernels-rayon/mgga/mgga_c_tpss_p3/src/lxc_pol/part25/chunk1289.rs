//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1289/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1289(t236: f64, t339: f64, t60698: f64, t18464: f64, t4480: f64, t1642: f64, t60706: f64, t18450: f64, t4462: f64, t60731: f64, t4473: f64, t60738: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65607 = t339 * t60698 * t236;
    let t65616 = t18464 * t4480;
    let t65624 = t60706 * t1642;
    let t65628 = t18450 * t4462;
    let t65634 = 35.0_f64 / 108.0_f64 * t60731;
    let t65639 = t60738 * t4473;
    (t65607, t65616, t65624, t65628, t65634, t65639)
}
