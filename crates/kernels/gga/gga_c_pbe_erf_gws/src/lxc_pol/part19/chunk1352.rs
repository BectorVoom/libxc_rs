//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1352/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1352<F: Float>(t1167: F, t11737: F, t11889: F, t13756: F, t14161: F, t14368: F, t14383: F, t15101: F, t3324: F, t3703: F, t3931: F, t3946: F, t4062: F, t4066: F, t52751: F, t54802: F, t54823: F, t54825: F, t54832: F, t54843: F, t54867: F, t56042: F, t57809: F) -> F {
    let t57942 = -F::cast_from(2.0_f64) * t1167 * t4062 * t54867 + F::cast_from(6.0_f64) * t11737 * t13756 * t4066 + F::cast_from(12.0_f64) * t11889 * t13756 * t4066 + F::cast_from(6.0_f64) * t13756 * t14161 * t3703 + F::cast_from(2.0_f64) * t14368 * t4062 * t56042 - F::cast_from(6.0_f64) * t14383 * t15101 * t3946 - F::cast_from(2.0_f64) * t15101 * t3324 * t4062 + F::cast_from(2.0_f64) * t3931 * t4062 * t52751 - F::cast_from(12.0_f64) * t54802 * t57809 + t54823 - t54825 - t54832 + t54843;
    t57942
}
