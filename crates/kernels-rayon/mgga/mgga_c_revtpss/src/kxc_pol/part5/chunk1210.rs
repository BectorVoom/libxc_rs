//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1210/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1210(t19226: f64, t954: f64, t11134: f64, t11574: f64, t15127: f64, t15189: f64, t15363: f64, t15364: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18944: f64, t18948: f64) -> (f64, f64) {
    let t19227 = t19226 * t954;
    let t19247 = -t11574 - 0.76103703703703703703e-2_f64 * t11134 - 0.1522074074074074074e-1_f64 * t15189 + 0.761037037037037037e-2_f64 * t15127 - t15363 + t15364 + 0.3805185185185185185e-2_f64 * t18919 - 0.19025925925925925925e-1_f64 * t18906 + 0.68493333333333333331e-1_f64 * t18911 - 0.2283111111111111111e-1_f64 * t18915 - 0.11415555555555555555e-1_f64 * t18924 - 0.10274e0_f64 * t18928 + 0.68493333333333333332e-1_f64 * t18932 + 0.57077777777777777777e-2_f64 * t18934 - 0.11415555555555555555e-1_f64 * t18939 + 0.34246666666666666666e-1_f64 * t18944 - 0.17123333333333333333e-1_f64 * t18948;
    (t19227, t19247)
}
