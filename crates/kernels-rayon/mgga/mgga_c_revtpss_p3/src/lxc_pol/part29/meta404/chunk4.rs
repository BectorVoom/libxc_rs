//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1459/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1459(t1626: f64, t3011: f64, t15125: f64, t15191: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11574: f64, t15127: f64, t15132: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15189: f64, t15195: f64) -> (f64, f64) {
    let t15350 = t1626 * t3011;
    let t15363 = 0.2283111111111111111e-1_f64 * t15125;
    let t15364 = 0.11415555555555555555e-1_f64 * t15191;
    let t15373 = -t11574 - 0.1522074074074074074e-1_f64 * t11134 + 0.38051851851851851851e-2_f64 * t11136 - 0.11415555555555555555e-1_f64 * t11138 + 0.57077777777777777777e-2_f64 * t11140 - 0.76103703703703703702e-2_f64 * t15189 + 0.76103703703703703701e-2_f64 * t15127 - t15363 + t15364 - 0.19025925925925925925e-1_f64 * t15142 + 0.68493333333333333331e-1_f64 * t15156 - 0.2283111111111111111e-1_f64 * t15132 - 0.11415555555555555555e-1_f64 * t15137 - 0.10274e0_f64 * t15160 + 0.68493333333333333332e-1_f64 * t15147 + 0.34246666666666666666e-1_f64 * t15151 - 0.17123333333333333333e-1_f64 * t15195;
    (t15350, t15373)
}
