//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 876/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk876<F: Float>(t3228: F, t6402: F, t6365: F, t904: F, t2083: F, t3037: F, t3165: F, t6: F, t1112: F, t2079: F, t2319: F, t3299: F) -> (F, F, F, F, F, F, F) {
    let t9342 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t6402 * t3228;
    let t9343 = t6365 * t904;
    let t9364 = t3037 * t2083;
    let t9375 = t6 * t3165;
    let t9385 = t2079 * t1112;
    let t9386 = t904 * t9385;
    let t9415 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t2319 * t3299;
    (t9342, t9343, t9364, t9375, t9385, t9386, t9415)
}
