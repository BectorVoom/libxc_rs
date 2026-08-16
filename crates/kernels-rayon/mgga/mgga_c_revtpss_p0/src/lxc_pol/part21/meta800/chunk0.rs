//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2901/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2901(t3011: f64, t4682: f64, t11506: f64, t1626: f64, t1609: f64, t2924: f64, t11112: f64, t2875: f64, t4632: f64, t11294: f64, t15098: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64, t51878: f64, t51881: f64, t51884: f64, t51887: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52637 = t4682 * t3011;
    let t52642 = t1626 * t11506;
    let t52645 = t2924 * t1609;
    let t52647 = 18.0_f64 * t52645 * t11112;
    let t52650 = 18.0_f64 * t2924 * t4632 * t2875;
    let t52652 = 18.0_f64 * t11294 * t15098;
    let t52664 = 0.71752e1_f64 * t51849 - 0.19931111111111111111e0_f64 * t51853 - 0.88582716049382716048e0_f64 * t51858 + 0.17938e1_f64 * t51863 + 0.17938e1_f64 * t51867 + 0.59793333333333333334e0_f64 * t51871 - 0.71752000000000000002e1_f64 * t51875 + 0.427258125e1_f64 * t51878 - 0.230371875e0_f64 * t51881 + 0.46074375e0_f64 * t51884 - 0.28483875e1_f64 * t51887;
    (t52637, t52642, t52647, t52650, t52652, t52664)
}
