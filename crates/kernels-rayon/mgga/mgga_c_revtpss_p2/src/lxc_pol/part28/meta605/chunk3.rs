//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2093/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2093(t25894: f64, t97676: f64, t97680: f64, t1444: f64, t5659: f64, t14110: f64, t94901: f64, t10073: f64, t1903: f64, t2029: f64, t25929: f64, t25930: f64, t25931: f64, t27868: f64, t49306: f64, t94635: f64, t94641: f64, t94648: f64, t94650: f64, t94662: f64, t94665: f64, t94672: f64, t94675: f64, t94677: f64) -> f64 {
    let t97838 = 0.28912093960683998208e-1_f64 * t25894 * t97676 * t97680;
    let t97839 = t5659 * t1444;
    let t97843 = t94901 * t14110;
    let t97847 = t10073 * t25929 * t2029 * t1903;
    let t97854 = 0.4336814094102599731e0_f64 * t27868 * t25931 * t49306 - 0.34270468708064099208e-1_f64 * t94635 + 0.12851425765524037203e-1_f64 * t94641 + t94648 - 0.51405703062096148812e-1_f64 * t94650 + t97838 - 0.17347256376410398924e1_f64 * t25930 * t25931 * t97839 + 0.39029762157531132075e-1_f64 * t97843 + 0.4818682326780666368e-3_f64 * t97847 + 0.38549458614245330943e-1_f64 * t94662 - 0.14456046980341999104e-1_f64 * t94665 - 0.77108554593144223218e-1_f64 * t94672 + 0.43368140941025997312e-1_f64 * t94675 + 0.34270468708064099208e-1_f64 * t94677;
    t97854
}
