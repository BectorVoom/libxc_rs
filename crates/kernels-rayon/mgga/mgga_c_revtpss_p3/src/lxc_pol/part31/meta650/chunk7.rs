//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2151/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2151(t19826: f64, t25509: f64, t20029: f64, t25505: f64, t100074: f64, t100255: f64, t1671: f64, t19651: f64, t19663: f64, t19668: f64, t19672: f64, t19930: f64, t19934: f64, t27536: f64, t4875: f64, t6312: f64, t7132: f64, t93655: f64) -> f64 {
    let t107015 = t25509 * t19826;
    let t107027 = t25505 * t20029;
    let t107035 = 0.22866142996303859718e-2_f64 * t93655 * t6312 - 0.28582678745379824648e-3_f64 * t107015 - 0.28582678745379824648e-2_f64 * t7132 * t19663 + 0.95275595817932748826e-3_f64 * t7132 * t19668 + 0.1270341277572436651e-2_f64 * t7132 * t19672 + 0.57165357490759649296e-3_f64 * t27536 * t19651 - 0.57165357490759649296e-3_f64 * t100255 * t4875 + 0.57165357490759649296e-3_f64 * t107027 + 0.17149607247227894789e-2_f64 * t7132 * t19930 - 0.11433071498151929859e-2_f64 * t7132 * t19934 - 0.45732285992607719437e-2_f64 * t100074 * t1671;
    t107035
}
