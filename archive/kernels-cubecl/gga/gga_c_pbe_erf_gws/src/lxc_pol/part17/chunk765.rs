//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 765/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk765<F: Float>(t252: F, t5385: F, t1907: F, t723: F, t1697: F, t212: F, t22: F, t1774: F, t586: F, t1651: F, t1655: F, t587: F) -> (F, F, F, F, F) {
    let t5387 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t252 * t5385;
    let t5388 = t1907 * t723;
    let t5399 = F::cast_from(1.0_f64) / t212 / t1697;
    let t5400 = t22 * t5399;
    let t5406 = t1774 * t586;
    let t5413 = t1651 * t1655;
    let t5414 = t587 * t5413;
    (t5387, t5388, t5400, t5406, t5414)
}
