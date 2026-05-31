//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 781/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk781<F: Float>(t339: F, t4379: F, t2178: F, t2181: F, t2183: F, t2186: F, t340: F, t6084: F, t6421: F, t6424: F, t6429: F, t6430: F, t6433: F, t870: F, t871: F) -> (F, F) {
    let t6436 = t339 * t4379;
    let t6439 = -t339 * t340 * t6084 + F::cast_from(9.0_f64) * t2178 * t2186 - F::cast_from(36.0_f64) * t2181 * t6433 - F::cast_from(36.0_f64) * t2183 * t6424 + F::cast_from(9.0_f64) * t6421 * t871 + F::cast_from(60.0_f64) * t6429 * t6430 + F::cast_from(3.0_f64) * t6436 * t870;
    (t6436, t6439)
}
