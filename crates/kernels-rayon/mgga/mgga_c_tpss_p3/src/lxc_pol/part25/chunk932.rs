//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 932/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk932(t3638: f64, t8313: f64, t236: f64, t339: f64, t8276: f64, t3678: f64, t219: f64, t3693: f64, t220: f64, t73: f64, t8275: f64, t3692: f64, t768: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10777 = 7.0_f64 / 576.0_f64 * t8313 * t3638;
    let t10779 = t339 * t8276 * t236;
    let t10803 = 7.0_f64 / 576.0_f64 * t8313 * t3678;
    let t10821 = t3693 * t219;
    let t10845 = t220 * t73 * t8275;
    let t10884 = t768 * t3692;
    (t10777, t10779, t10803, t10821, t10845, t10884)
}
