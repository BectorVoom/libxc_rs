//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2866/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2866(t15199: f64, t698: f64, t141: f64, t51969: f64, t930: f64, t51973: f64, t41329: f64, t41361: f64, t41363: f64, t41369: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64, t51961: f64, t51965: f64, t51967: f64, t51971: f64, t51978: f64, t52028: f64, t52031: f64, t52033: f64) -> (f64, f64, f64) {
    let t52065 = t698 * t15199;
    let t52068 = t141 * t930 * t51969;
    let t52082 = 4.0_f64 / 9.0_f64 * t51973;
    let t52090 = 8.0_f64 * t51849 - 2.0_f64 / 9.0_f64 * t51853 - 80.0_f64 / 81.0_f64 * t51858 + 2.0_f64 * t51863 + 2.0_f64 * t51867 + 2.0_f64 / 3.0_f64 * t51871 - 8.0_f64 * t51875 + t41329 + 4.0_f64 * t51961 - 10.0_f64 / 9.0_f64 * t51965 + t51967 / 3.0_f64 - t51971 / 3.0_f64 - t52082 + 28.0_f64 / 81.0_f64 * t51978 + 28.0_f64 / 27.0_f64 * t41361 + 8.0_f64 / 9.0_f64 * t41363 - 4.0_f64 / 9.0_f64 * t41369 + 4.0_f64 * t52028 + 40.0_f64 / 9.0_f64 * t52031 + 2.0_f64 * t52033;
    (t52065, t52068, t52090)
}
