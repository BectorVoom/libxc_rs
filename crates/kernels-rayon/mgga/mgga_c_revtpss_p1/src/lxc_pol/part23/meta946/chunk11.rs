//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3127/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3127(t12910: f64, t17459: f64, t17753: f64, t1808: f64, t20952: f64, t21004: f64, t21014: f64, t21017: f64, t21030: f64, t21173: f64, t21242: f64, t24751: f64, t3626: f64, t3629: f64, t3720: f64, t5397: f64, t57147: f64, t57382: f64, t69661: f64, t69710: f64, t82286: f64, t82289: f64, t82293: f64) -> f64 {
    let t82305 = 0.12862205435420921092e-2_f64 * t57382 * t21030 + 0.12862205435420921092e-2_f64 * t12910 * t3720 * t24751 * t17459 - 0.13719685797782315831e-1_f64 * t57147 * t21004 - 0.57165357490759649296e-3_f64 * t82286 + 0.28582678745379824648e-3_f64 * t82289 - 0.22866142996303859718e-2_f64 * t21017 * t21173 - 0.14291339372689912324e-3_f64 * t17753 * t3626 * t82293 * t3629 + 0.95275595817932748827e-4_f64 * t69661 + 0.45732285992607719436e-2_f64 * t69710 * t1808 + 0.45732285992607719436e-2_f64 * t21242 * t5397 - 0.13719685797782315831e-1_f64 * t21014 * t20952;
    t82305
}
