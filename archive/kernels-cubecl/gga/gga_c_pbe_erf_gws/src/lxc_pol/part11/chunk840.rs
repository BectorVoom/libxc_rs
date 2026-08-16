//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 840/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk840<F: Float>(t11459: F, t2170: F, t3814: F, t2168: F, t1112: F, t816: F, t11994: F, t3257: F, t1109: F) -> (F, F, F, F, F) {
    let t13282 = t2170 * t11459 * t3814;
    let t13284 = t2168 * t13282 / F::cast_from(16.0_f64);
    let t13285 = t816 * t1112;
    let t13287 = t3257 * t11994 * t13285;
    let t13290 = t1109 * t1109;
    (t13282, t13284, t13285, t13287, t13290)
}
