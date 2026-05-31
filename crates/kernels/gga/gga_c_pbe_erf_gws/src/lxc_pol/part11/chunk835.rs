//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 835/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk835<F: Float>(t13220: F, t376: F, t338: F, t353: F, t1161: F, t3717: F, t2376: F, t2409: F, t11630: F, t3123: F, t11778: F, t11794: F, t3134: F) -> (F, F, F, F, F, F, F) {
    let t13221 = t376 * t13220;
    let t13223 = t338 * t353 * t13221;
    let t13227 = t3717 * t1161;
    let t13229 = t2409 * t2376 * t13227;
    let t13233 = t3123 * t11630 / F::cast_from(32.0_f64);
    let t13235 = t3123 * t11778 / F::cast_from(32.0_f64);
    let t13237 = t11794 * t3134 / F::cast_from(32.0_f64);
    (t13221, t13223, t13227, t13229, t13233, t13235, t13237)
}
