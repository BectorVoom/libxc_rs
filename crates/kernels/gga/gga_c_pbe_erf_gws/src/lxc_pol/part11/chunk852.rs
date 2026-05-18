//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 852/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk852<F: Float>(t13408: F, t3219: F, t6366: F, t11610: F, t12041: F, t860: F, t11514: F, t13403: F, t2345: F, t1076: F, t274: F, t3257: F, t3803: F) -> (F, F, F, F, F) {
    let t13410 = t6366 * t3219 * t13408;
    let t13414 = t12041 * t11610;
    let t13416 = t13414 * t860 / F::new(32.0);
    let t13418 = t2345 * t11514 * t13403;
    let t13421 = t1076 * t274;
    let t13423 = t3257 * t3803 * t13421;
    (t13410, t13414, t13416, t13418, t13423)
}
