//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3519/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3519(t15700: f64, t19992: f64, t53405: f64, t16226: f64, t19997: f64, t11710: f64, t19777: f64, t3091: f64, t19644: f64, t15596: f64, t15605: f64, t15611: f64, t15618: f64, t15688: f64, t1664: f64, t19707: f64, t19722: f64, t42967: f64, t43043: f64, t4912: f64, t53800: f64, t53855: f64, t54289: f64, t54341: f64, t54348: f64, t54542: f64, t6268: f64) -> f64 {
    let t66644 = t15700 * t53405 * t19992;
    let t66647 = t16226 * t53405 * t19997;
    let t66655 = t3091 * t11710 * t19777;
    let t66660 = t3091 * t11710 * t19644;
    let t66662 = -0.17149607247227894789e-2_f64 * t43043 * t15688 * t1664 * t15605 - 0.60976381323476959249e-2_f64 * t54289 * t19707 + 0.42874018118069736972e-3_f64 * t54542 * t19722 - 0.85748036236139473944e-3_f64 * t53855 * t4912 - 0.3811023832717309953e-3_f64 * t54341 - 0.76220476654346199061e-3_f64 * t66644 + 0.76220476654346199061e-3_f64 * t66647 + 0.76220476654346199061e-3_f64 * t54348 - 0.30488190661738479624e-2_f64 * t42967 * t6268 - 0.17149607247227894789e-2_f64 * t53800 * t15611 + 0.3811023832717309953e-3_f64 * t66655 + 0.47637797908966374413e-3_f64 * t15618 * t15596 + 0.19055119163586549765e-3_f64 * t66660;
    t66662
}
