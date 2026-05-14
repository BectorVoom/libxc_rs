//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1085/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1085<F: Float>(t45128: F, t9016: F, t13525: F, t37768: F, t2157: F, t3703: F, t11478: F, t3138: F, t6523: F, t13220: F, t20842: F, t2168: F, t3131: F, t343: F, t45048: F, t37994: F) -> (F, F, F, F, F, F, F) {
    let t49561 = t9016 * t45128 / 4.0;
    let t49567 = t37768 * t13525 / 12.0;
    let t49568 = t2157 * t3703;
    let t49572 = 3.0 / 4.0 * t3138 * t6523 * t11478 * t49568;
    let t49576 = t2168 * t20842 * t3131 * t343 * t13220;
    let t49577 = 7.0 / 24.0 * t45048;
    let t49579 = t37994 * t13525 / 8.0;
    (t49561, t49567, t49568, t49572, t49576, t49577, t49579)
}
