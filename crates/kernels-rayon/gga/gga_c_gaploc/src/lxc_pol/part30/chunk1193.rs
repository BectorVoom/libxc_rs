//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1193/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1193(t10243: f64, t6313: f64, t1063: f64, t6750: f64, t7930: f64, t1222: f64, t3351: f64, t2321: f64, t8289: f64, t882: f64, t10156: f64, t31928: f64, t31930: f64, t31932: f64, t31935: f64, t31939: f64, t31942: f64, t31945: f64, t31948: f64, t31952: f64, t31956: f64, t4807: f64) -> f64 {
    let t31958 = 0.7588001769513639893e-1_f64 * t6313 * t10243;
    let t31961 = 0.17073003981405689759e0_f64 * t1063 * t7930 * t6750;
    let t31965 = t1222 * t3351;
    let t31966 = 0.31616674039640166222e-2_f64 * t31965;
    let t31968 = t882 * t8289 * t2321;
    let t31969 = 0.11856252764865062333e-2_f64 * t31968;
    let t31970 = -t31928 + t31930 - t31932 - t31935 - t31939 + t31942 + t31945 - t31948 - t31952 - t31956 + t31958 + t31961 + 0.17073003981405689759e0_f64 * t1063 * t10156 * t4807 + t31966 + t31969;
    t31970
}
