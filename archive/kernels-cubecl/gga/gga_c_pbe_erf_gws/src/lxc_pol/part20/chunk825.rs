//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 825/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk825<F: Float>(t7380: F, t2723: F, t582: F, t211: F, t2519: F, t713: F, t1652: F, t2615: F, t1009: F, t4991: F, t587: F, t2815: F, t586: F) -> (F, F, F, F, F, F) {
    let t7549 = F::cast_from(0.2518888888888888889e-2_f64) * t7380;
    let t7570 = t582 * t2723;
    let t7572 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t211 * t7570;
    let t7573 = t2519 * t713;
    let t7578 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t2615 * t1652;
    let t7579 = t4991 * t1009;
    let t7580 = t587 * t7579;
    let t7582 = t2815 * t586;
    (t7549, t7572, t7573, t7578, t7580, t7582)
}
