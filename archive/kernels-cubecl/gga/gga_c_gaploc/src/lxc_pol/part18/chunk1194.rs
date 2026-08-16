//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1194/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1194<F: Float>(t10243: F, t6313: F, t1063: F, t6750: F, t7930: F, t1222: F, t3351: F, t2321: F, t8289: F, t882: F, t10156: F, t31928: F, t31930: F, t31932: F, t31935: F, t31939: F, t31942: F, t31945: F, t31948: F, t31952: F, t31956: F, t4807: F) -> F {
    let t31958 = F::cast_from(0.7588001769513639893e-1_f64) * t6313 * t10243;
    let t31961 = F::cast_from(0.17073003981405689759e0_f64) * t1063 * t7930 * t6750;
    let t31965 = t1222 * t3351;
    let t31966 = F::cast_from(0.31616674039640166222e-2_f64) * t31965;
    let t31968 = t882 * t8289 * t2321;
    let t31969 = F::cast_from(0.11856252764865062333e-2_f64) * t31968;
    let t31970 = -t31928 + t31930 - t31932 - t31935 - t31939 + t31942 + t31945 - t31948 - t31952 - t31956 + t31958 + t31961 + F::cast_from(0.17073003981405689759e0_f64) * t1063 * t10156 * t4807 + t31966 + t31969;
    t31970
}
