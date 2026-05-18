//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 828/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk828<F: Float>(t2053: F, t944: F, t2096: F, t2454: F, t4344: F, t4498: F, t19: F, t3025: F, t796: F, t801: F, t1105: F, t945: F) -> (F, F, F, F, F, F) {
    let t6868 = t944 * t2053;
    let t6906 = t2454 * t2096;
    let t6907 = F::new(0.6846054806677777778e0) * t6906;
    let t6911 = F::new(0.41076328840066666668e0) * t4344;
    let t6918 = F::new(4.0) * t4498;
    let t6921 = t3025 * t796 * t19;
    let t6922 = t6921 * t801;
    let t6923 = F::new(0.82152657680133333336e0) * t6922;
    let t6925 = t945 * t1105;
    (t6868, t6907, t6911, t6918, t6923, t6925)
}
