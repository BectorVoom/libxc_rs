//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 666/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk666<F: Float>(t5304: F, t593: F, t1406: F, t597: F, t610: F, t1885: F, t1820: F, t1878: F, t586: F) -> (F, F, F, F, F, F) {
    let t5306 = F::new(8.0) / F::new(15.0) * t5304 * t593;
    let t5307 = t597 * t1406;
    let t5308 = t5307 * t610;
    let t5309 = t1885 * t5308;
    let t5311 = F::new(4.0) / F::new(5.0) * t1820 * t5309;
    let t5312 = t1878 * t586;
    (t5306, t5307, t5308, t5309, t5311, t5312)
}
