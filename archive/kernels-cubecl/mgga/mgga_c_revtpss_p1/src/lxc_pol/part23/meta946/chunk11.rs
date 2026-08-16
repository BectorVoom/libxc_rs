//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3127/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3127<F: Float>(t12910: F, t17459: F, t17753: F, t1808: F, t20952: F, t21004: F, t21014: F, t21017: F, t21030: F, t21173: F, t21242: F, t24751: F, t3626: F, t3629: F, t3720: F, t5397: F, t57147: F, t57382: F, t69661: F, t69710: F, t82286: F, t82289: F, t82293: F) -> F {
    let t82305 = F::cast_from(0.12862205435420921092e-2_f64) * t57382 * t21030 + F::cast_from(0.12862205435420921092e-2_f64) * t12910 * t3720 * t24751 * t17459 - F::cast_from(0.13719685797782315831e-1_f64) * t57147 * t21004 - F::cast_from(0.57165357490759649296e-3_f64) * t82286 + F::cast_from(0.28582678745379824648e-3_f64) * t82289 - F::cast_from(0.22866142996303859718e-2_f64) * t21017 * t21173 - F::cast_from(0.14291339372689912324e-3_f64) * t17753 * t3626 * t82293 * t3629 + F::cast_from(0.95275595817932748827e-4_f64) * t69661 + F::cast_from(0.45732285992607719436e-2_f64) * t69710 * t1808 + F::cast_from(0.45732285992607719436e-2_f64) * t21242 * t5397 - F::cast_from(0.13719685797782315831e-1_f64) * t21014 * t20952;
    t82305
}
