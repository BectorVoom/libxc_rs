//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 662/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk662<F: Float>(t2332: F, t899: F, t900: F, t329: F, t6594: F, t378: F, t4383: F, t824: F, t6472: F, t825: F, t332: F, t931: F) -> (F, F, F, F, F, F) {
    let t6717 = t899 * t900 * t2332;
    let t6729 = t329 * t6594;
    let t6731 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t6729 * t378;
    let t6792 = t824 * t4383;
    let t6801 = t6472 * t825;
    let t6816 = t329 * t332 * t931;
    (t6717, t6729, t6731, t6792, t6801, t6816)
}
