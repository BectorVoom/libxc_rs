//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1153;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1154;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta287(t11239: f64, t3143: f64, t342: f64, t12051: f64, t3154: f64, t12048: f64, t1071: f64, t3151: f64, t3304: f64, t3318: f64, t11687: f64, t4998: f64, t1086: f64, t3043: f64, t3075: f64, t3291: f64, t1082: f64, t11202: f64, t1024: f64, t1087: f64, t1090: f64, t1093: f64, t11788: f64, t11902: f64, t11940: f64, t12047: f64, t12053: f64, t12057: f64, t12066: f64, t12070: f64, t12074: f64, t3278: f64, t3283: f64, t3299: f64, t3309: f64, t3313: f64, t3317: f64, t3322: f64, t381: f64, t4996: f64, t989: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12077, t12078, t12079, t12080, t12086, t12089, t12094) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1153(t11239, t3143, t342, t12051, t3154, t12048, t1071, t3151, t3304, t3318, t11687, t4998);
        let (t12097, t12100, t12105, t12108) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1154(t1086, t3043, t3075, t3291, t1082, t11202, t1024, t1087, t1090, t1093, t11788, t11902, t11940, t12047, t12053, t12057, t12066, t12070, t12074, t12078, t12080, t12086, t12089, t12094, t3278, t3283, t3299, t3309, t3313, t3317, t3322, t342, t381, t4996, t989);
    (t12077, t12078, t12079, t12080, t12086, t12089, t12094, t12097, t12100, t12105, t12108)
}
