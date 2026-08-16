//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2083/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2083(t14104: f64, t94725: f64, t1358: f64, t2439: f64, t785: f64, t7910: f64, t2435: f64, t7925: f64, t25904: f64, t13920: f64, t14224: f64, t2022: f64, t25930: f64, t25931: f64, t25933: f64, t27864: f64, t27868: f64, t27980: f64, t27981: f64, t4056: f64, t49380: f64, t543: f64, t7295: f64, t7301: f64, t94682: f64, t94694: f64, t94716: f64, t97855: f64, t97858: f64, t97869: f64, t97871: f64, t97875: f64) -> (f64, f64) {
    let t97882 = t94725 * t14104;
    let t97894 = t2439 * t785 * t7910 * t1358;
    let t97899 = t7925 * t2435;
    let t97900 = t25904 * t97899;
    let t97903 = -0.17347256376410398924e1_f64 * t97855 * t27981 - 0.8673628188205199462e0_f64 * t25930 * t25931 * t97858 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t2022 * t13920 * t543 + t97869 + 0.17347256376410398924e1_f64 * t25930 * t27980 * t97871 - 0.17347256376410398924e1_f64 * t25930 * t97875 * t25933 + 0.4336814094102599731e0_f64 * t27868 * t25931 * t49380 - 0.11565819519348392139e-2_f64 * t97882 + 0.8673628188205199462e0_f64 * t27868 * t94716 * t14224 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t7910 * t4056 * t543 - 0.65049603595885220126e-3_f64 * t97894 - 0.17347256376410398924e1_f64 * t25930 * t94716 * t27864 + 0.96373646535613327357e-2_f64 * t97900 + t94682 + 0.10975748638225852664e-1_f64 * t94694;
    (t97899, t97903)
}
