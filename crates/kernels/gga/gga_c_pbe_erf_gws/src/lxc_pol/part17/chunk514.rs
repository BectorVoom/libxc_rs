//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 514/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk514<F: Float>(t2135: F, t2170: F, t2171: F, t2168: F, t369: F, t814: F, t322: F, t931: F, t810: F) -> (F, F, F, F, F) {
    let t2173 = t2170 * t2135 * t2171;
    let t2175 = t2168 * t2173 / F::cast_from(24.0_f64);
    let t2178 = t814 * t369;
    let t2181 = t322 * t931;
    let t2182 = t810 * t810;
    (t2173, t2175, t2178, t2181, t2182)
}
