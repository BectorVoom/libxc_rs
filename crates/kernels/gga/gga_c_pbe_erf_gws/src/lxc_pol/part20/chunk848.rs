//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 848/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk848<F: Float>(t1371: F, t2948: F, t553: F, t1378: F, t1971: F, t8361: F, t163: F, t169: F, t2979: F, t299: F, t2962: F, t679: F) -> (F, F, F, F) {
    let t8387 = t2948 * t1371 * t553;
    let t8390 = t8361 * t1378 * t1971;
    let t8395 = F::cast_from(0.17961351015381913641e-1_f64) * t169 * t299 * t2979 * t163;
    let t8404 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2962 * t679;
    (t8387, t8390, t8395, t8404)
}
