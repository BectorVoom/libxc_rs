//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1153;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1154;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta287<F: Float>(t11239: F, t3143: F, t342: F, t12051: F, t3154: F, t12048: F, t1071: F, t3151: F, t3304: F, t3318: F, t11687: F, t4998: F, t1086: F, t3043: F, t3075: F, t3291: F, t1082: F, t11202: F, t1024: F, t1087: F, t1090: F, t1093: F, t11788: F, t11902: F, t11940: F, t12047: F, t12053: F, t12057: F, t12066: F, t12070: F, t12074: F, t3278: F, t3283: F, t3299: F, t3309: F, t3313: F, t3317: F, t3322: F, t381: F, t4996: F, t989: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12077, t12078, t12079, t12080, t12086, t12089, t12094) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1153::<F>(t11239, t3143, t342, t12051, t3154, t12048, t1071, t3151, t3304, t3318, t11687, t4998);
        let (t12097, t12100, t12105, t12108) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1154::<F>(t1086, t3043, t3075, t3291, t1082, t11202, t1024, t1087, t1090, t1093, t11788, t11902, t11940, t12047, t12053, t12057, t12066, t12070, t12074, t12078, t12080, t12086, t12089, t12094, t3278, t3283, t3299, t3309, t3313, t3317, t3322, t342, t381, t4996, t989);
    (t12077, t12078, t12079, t12080, t12086, t12089, t12094, t12097, t12100, t12105, t12108)
}
