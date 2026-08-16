//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3722/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3722(t480: f64, t69637: f64, t20842: f64, t3667: f64, t17303: f64, t5323: f64, t12784: f64, t17401: f64, t17484: f64, t17515: f64, t17534: f64, t17654: f64, t17662: f64, t17729: f64, t17744: f64, t20766: f64, t21161: f64, t3626: f64, t3674: f64, t5051: f64, t56981: f64, t57331: f64, t57333: f64, t57336: f64, t57660: f64, t57663: f64, t57710: f64) -> f64 {
    let t70578 = t69637 * t480;
    let t70581 = t3667 * t20842;
    let t70583 = t5323 * t17303;
    let t70593 = -0.57165357490759649296e-3_f64 * t12784 * t21161 + 0.1270341277572436651e-3_f64 * t57331 - 0.3811023832717309953e-3_f64 * t57333 - 0.57165357490759649296e-3_f64 * t57336 + 0.11433071498151929859e-2_f64 * t17729 * t3626 * t5051 * t17534 - 0.11433071498151929859e-2_f64 * t17654 * t56981 * t20766 + 0.42874018118069736972e-3_f64 * t70578 * t3674 - 0.28582678745379824648e-3_f64 * t70581 - 0.5081365110289746604e-3_f64 * t70583 - 0.22866142996303859718e-2_f64 * t57710 * t17484 - 0.42874018118069736972e-3_f64 * t17401 * t17744 - 0.30488190661738479624e-2_f64 * t57660 * t17662 + 0.57165357490759649296e-3_f64 * t57663 * t17515;
    t70593
}
