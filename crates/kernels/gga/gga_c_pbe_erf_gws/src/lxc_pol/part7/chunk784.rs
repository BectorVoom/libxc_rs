//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 784/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk784<F: Float>(t16329: F, t16331: F, t16334: F, t16335: F, t16336: F, t16337: F, t16338: F, t16340: F, t16345: F, t4344: F, t4381: F, t4545: F, t4547: F, t4602: F, t6068: F, t6839: F, t6841: F) -> (F,) {
    let t16346 = t16329 - 0.49291594608080000001e1 * t4344 - t16331 + 12.0 * t4381 + 4.0 * t6839 + t16334 + t16335 - t16336 - t16337 + t16338 - 36.0 * t6841 + t16340 - 0.75926915593978166528e1 * t4545 - 48.0 * t4547 - 4.0 * t6068 + 12.0 * t4602 + t16345;
    (t16346,)
}
