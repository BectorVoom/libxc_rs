//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1310/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1310<F: Float>(t14193: F, t22493: F, t53060: F, t14185: F, t3306: F, t353: F, t859: F, t1105: F, t4111: F, t4386: F, t1206: F, t2494: F) -> (F, F, F, F, F) {
    let t54942 = F::new(7.0) / F::new(144.0) * t22493 * t14193;
    let t54946 = F::new(7.0) / F::new(288.0) * t53060;
    let t54952 = t859 * t353 * t14185 * t3306;
    let t54957 = t4386 * t353 * t4111 * t1105;
    let t54962 = t4386 * t353 * t1206 * t2494;
    (t54942, t54946, t54952, t54957, t54962)
}
