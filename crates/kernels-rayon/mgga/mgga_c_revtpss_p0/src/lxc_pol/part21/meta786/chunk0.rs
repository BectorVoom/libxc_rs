//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2833/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2833(t11354: f64, t2881: f64, t4606: f64, t11358: f64, t15220: f64, t2897: f64, t918: f64, t2880: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64) -> (f64, f64, f64, f64, f64) {
    let t51878 = t11354 * t4606 * t2881;
    let t51881 = t11358 * t4606 * t2881;
    let t51884 = t2897 * t15220 * t918;
    let t51887 = t2880 * t15220 * t918;
    let t51889 = 0.72462e1_f64 * t51849 - 0.20128333333333333333e0_f64 * t51853 - 0.89459259259259259259e0_f64 * t51858 + 0.181155e1_f64 * t51863 + 0.181155e1_f64 * t51867 + 0.60385e0_f64 * t51871 - 0.72462e1_f64 * t51875 + 0.58258125e1_f64 * t51878 - 0.1237865625e0_f64 * t51881 + 0.247573125e0_f64 * t51884 - 0.3883875e1_f64 * t51887;
    (t51878, t51881, t51884, t51887, t51889)
}
