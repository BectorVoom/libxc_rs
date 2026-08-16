//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1147/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1147(t121884: f64, t32474: f64, t119808: f64, t8477: f64, t32469: f64, t121834: f64, t31837: f64, t93169: f64, t119903: f64, t121808: f64, t31830: f64, t25413: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121886 = 0.33852964522850660984e-1_f64 * t32474 * t121884;
    let t121887 = t8477 * t119808;
    let t121891 = 0.19039912555034117539e-1_f64 * t32469 * t121884;
    let t121896 = 0.95199562775170587692e-3_f64 * t93169 * t31837 * t121834;
    let t121897 = 0.37645955677973955999e-5_f64 * t119903;
    let t121901 = t31830 * t121808;
    let t121902 = t121901 * t25413;
    (t121886, t121887, t121891, t121896, t121897, t121901, t121902)
}
