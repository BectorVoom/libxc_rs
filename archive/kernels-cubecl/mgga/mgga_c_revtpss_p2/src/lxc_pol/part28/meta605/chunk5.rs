//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2095/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2095<F: Float>(t14104: F, t94725: F, t1358: F, t2439: F, t785: F, t7910: F, t2435: F, t7925: F, t25904: F, t13920: F, t14224: F, t2022: F, t25930: F, t25931: F, t25933: F, t27864: F, t27868: F, t27980: F, t27981: F, t4056: F, t49380: F, t543: F, t7295: F, t7301: F, t94682: F, t94694: F, t94716: F, t97855: F, t97858: F, t97869: F, t97871: F, t97875: F) -> (F, F) {
    let t97882 = t94725 * t14104;
    let t97894 = t2439 * t785 * t7910 * t1358;
    let t97899 = t7925 * t2435;
    let t97900 = t25904 * t97899;
    let t97903 = -F::cast_from(0.17347256376410398924e1_f64) * t97855 * t27981 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t25931 * t97858 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t2022 * t13920 * t543 + t97869 + F::cast_from(0.17347256376410398924e1_f64) * t25930 * t27980 * t97871 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t97875 * t25933 + F::cast_from(0.4336814094102599731e0_f64) * t27868 * t25931 * t49380 - F::cast_from(0.11565819519348392139e-2_f64) * t97882 + F::cast_from(0.8673628188205199462e0_f64) * t27868 * t94716 * t14224 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t7910 * t4056 * t543 - F::cast_from(0.65049603595885220126e-3_f64) * t97894 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t94716 * t27864 + F::cast_from(0.96373646535613327357e-2_f64) * t97900 + t94682 + F::cast_from(0.10975748638225852664e-1_f64) * t94694;
    (t97899, t97903)
}
