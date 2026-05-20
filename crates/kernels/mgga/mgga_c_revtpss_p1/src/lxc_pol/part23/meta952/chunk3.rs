//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3158/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3158<F: Float>(t1042: F, t1261: F, t12787: F, t17569: F, t17736: F, t20792: F, t20800: F, t20811: F, t20950: F, t21143: F, t3362: F, t3720: F, t4181: F, t5302: F, t5304: F, t5340: F, t5381: F, t57056: F, t6573: F, t6631: F, t69971: F, t69984: F, t70006: F, t70008: F, t78770: F) -> F {
    let t82978 = F::cast_from(0.15244095330869239812e-2_f64) * t69971 + F::cast_from(0.95275595817932748826e-3_f64) * t69984 + F::cast_from(0.7145669686344956162e-3_f64) * t21143 * t5304 + F::cast_from(0.7145669686344956162e-3_f64) * t5381 * t20792 + F::cast_from(0.23818898954483187207e-3_f64) * t1261 * t1042 * t5302 * t78770 - F::cast_from(0.28582678745379824648e-3_f64) * t70006 + F::cast_from(0.30488190661738479624e-2_f64) * t70008 + F::cast_from(0.14291339372689912324e-2_f64) * t17736 * t12787 * t6573 * t3362 * t4181 + F::cast_from(0.12862205435420921092e-2_f64) * t5340 * t3720 * t20800 * t20950 - F::cast_from(0.68598428988911579154e-2_f64) * t57056 * t6631 + F::cast_from(0.42874018118069736972e-3_f64) * t17569 * t20811;
    t82978
}
