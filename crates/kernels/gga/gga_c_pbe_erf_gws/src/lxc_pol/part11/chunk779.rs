//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 779/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk779<F: Float>(t13347: F, t2345: F, t3219: F, t2118: F, t3786: F, t3912: F, t860: F, t2255: F, t3752: F, t3781: F, t11564: F, t3180: F, t11499: F, t11557: F, t3257: F, t11773: F, t3134: F) -> (F, F, F, F, F, F, F) {
    let t13349 = t2345 * t3219 * t13347;
    let t13352 = t2118 * t3786;
    let t13353 = t3912 * t13352;
    let t13355 = t13353 * t860 / 32.0;
    let t13357 = t2255 * t3781 * t3752;
    let t13361 = t11564 * t3180 / 16.0;
    let t13363 = t3257 * t11499 * t11557;
    let t13367 = t11773 * t3134 / 32.0;
    (t13349, t13353, t13355, t13357, t13361, t13363, t13367)
}
