//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 842/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk842<F: Float>(t13304: F, t2168: F, t11808: F, t3128: F, t1149: F, t11700: F, t11592: F, t3793: F, t11493: F, t13220: F, t339: F, t1130: F, t3717: F) -> (F, F, F, F, F, F, F) {
    let t13306 = t2168 * t13304 / F::cast_from(32.0_f64);
    let t13308 = t3128 * t11808 / F::cast_from(16.0_f64);
    let t13309 = t11700 * t1149;
    let t13313 = t11592 * t3793 / F::cast_from(48.0_f64);
    let t13314 = F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t11493;
    let t13325 = t339 * t13220;
    let t13328 = t1130 * t3717;
    (t13306, t13308, t13309, t13313, t13314, t13325, t13328)
}
