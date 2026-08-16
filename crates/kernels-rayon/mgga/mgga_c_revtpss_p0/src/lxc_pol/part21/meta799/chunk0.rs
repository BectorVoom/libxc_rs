//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2893/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2893(t15494: f64, t964: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64, t51878: f64, t51881: f64, t51884: f64, t51887: f64) -> (f64, f64) {
    let t52522 = t15494 * t964;
    let t52536 = 0.123954e2_f64 * t51849 - 0.34431666666666666667e0_f64 * t51853 - 0.15302962962962962963e1_f64 * t51858 + 0.309885e1_f64 * t51863 + 0.309885e1_f64 * t51867 + 0.103295e1_f64 * t51871 - 0.123954e2_f64 * t51875 + 0.794188125e1_f64 * t51878 - 0.473371875e0_f64 * t51881 + 0.94674375e0_f64 * t51884 - 0.52945875e1_f64 * t51887;
    (t52522, t52536)
}
