//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2077/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2077(t2022: f64, t9990: f64, t1426: f64, t786: f64, t7911: f64, t3917: f64, t14230: f64, t25924: f64, t25926: f64, t27837: f64, t27868: f64, t27973: f64, t27980: f64, t3999: f64, t4077: f64, t4131: f64, t48020: f64, t48074: f64, t49393: f64, t7274: f64, t7295: f64, t7296: f64, t7910: f64, t7920: f64, t94593: f64, t94598: f64, t94602: f64, t94605: f64, t94656: f64, t94705: f64) -> f64 {
    let t97764 = t9990 * t2022;
    let t97783 = t786 * t7911 * t1426;
    let t97785 = 0.19514881078765566038e-1_f64 * t97783 * t3917;
    let t97791 = 0.8673628188205199462e0_f64 * t7295 * t7296 * t7910 * t4131 + 0.34270468708064099208e-1_f64 * t94593 - 0.28912093960683998208e-1_f64 * t94598 + 0.10408353825846239354e2_f64 * t7295 * t94656 * t7920 * t4077 + 0.26020884564615598386e1_f64 * t27868 * t97764 * t49393 - 0.26020884564615598386e1_f64 * t27868 * t27980 * t48074 - 0.17347256376410398924e1_f64 * t27868 * t3999 * t7274 * t14230 - 0.26020884564615598386e1_f64 * t7295 * t25924 * t7910 * t4077 - 0.17347256376410398924e1_f64 * t27868 * t27980 * t48020 - t97785 - 0.26020884564615598386e1_f64 * t27837 * t25926 + t94602 - 0.14456046980341999104e-1_f64 * t94605 - 0.17347256376410398924e1_f64 * t94705 * t27973;
    t97791
}
