//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1352/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1352(t1167: f64, t11737: f64, t11889: f64, t13756: f64, t14161: f64, t14368: f64, t14383: f64, t15101: f64, t3324: f64, t3703: f64, t3931: f64, t3946: f64, t4062: f64, t4066: f64, t52751: f64, t54802: f64, t54823: f64, t54825: f64, t54832: f64, t54843: f64, t54867: f64, t56042: f64, t57809: f64) -> f64 {
    let t57942 = -2.0_f64 * t1167 * t4062 * t54867 + 6.0_f64 * t11737 * t13756 * t4066 + 12.0_f64 * t11889 * t13756 * t4066 + 6.0_f64 * t13756 * t14161 * t3703 + 2.0_f64 * t14368 * t4062 * t56042 - 6.0_f64 * t14383 * t15101 * t3946 - 2.0_f64 * t15101 * t3324 * t4062 + 2.0_f64 * t3931 * t4062 * t52751 - 12.0_f64 * t54802 * t57809 + t54823 - t54825 - t54832 + t54843;
    t57942
}
