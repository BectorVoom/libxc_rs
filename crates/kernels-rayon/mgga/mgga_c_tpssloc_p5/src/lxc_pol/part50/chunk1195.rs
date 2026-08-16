//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1195/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1195(t112943: f64, t23164: f64, t7479: f64, t1880: f64, t25224: f64, t30656: f64, t113038: f64, t113045: f64, t118916: f64, t118917: f64, t118918: f64, t118924: f64, t118928: f64, t118935: f64, t118938: f64, t23281: f64, t25188: f64, t25233: f64, t25330: f64, t2597: f64, t32853: f64, t6627: f64, t6663: f64, t7538: f64) -> f64 {
    let t118940 = t23164 * t112943 * t7479;
    let t118941 = 0.16449340668482264365e-1_f64 * t118940;
    let t118944 = 0.16449340668482264365e-1_f64 * t1880 * t25224 * t30656;
    let t118945 = -2.0_f64 * t23281 * t7538 - 2.0_f64 * t25188 * t6663 + 4.0_f64 * t25233 * t6627 - 2.0_f64 * t25330 * t6627 - t2597 * t32853 + t113038 - t113045 + t118916 + t118917 + t118918 - t118924 + t118928 - t118935 - t118938 + t118941 - t118944;
    t118945
}
