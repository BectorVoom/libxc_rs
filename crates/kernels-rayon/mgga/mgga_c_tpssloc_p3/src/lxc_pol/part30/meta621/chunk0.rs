//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2020/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2020(t2: f64, t2752: f64, t193: f64, t201: f64, t7540: f64, t870: f64, t25353: f64, t25213: f64, t6547: f64, t4119: f64, t857: f64, t23168: f64, t25342: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86730 = t2752 * t2;
    let t86736 = t193 * t201 * t7540;
    let t86753 = t870 * t2;
    let t86836 = t25353 * t2752;
    let t86843 = t6547 * t25213;
    let t86844 = 0.38381794893125283518e-1_f64 * t86843;
    let t86849 = t857 * t4119;
    let t86868 = t23168 * t25342;
    (t86730, t86736, t86753, t86836, t86844, t86849, t86868)
}
