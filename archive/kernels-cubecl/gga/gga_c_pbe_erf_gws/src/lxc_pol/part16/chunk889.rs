//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 889/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk889<F: Float>(t587: F, t7695: F, t1660: F, t331: F, t197: F, t7346: F, t1802: F, t1885: F, t1017: F, t562: F, t610: F, t1820: F) -> (F, F, F) {
    let t7697 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t587 * t7695;
    let t7698 = t331 * t1660;
    let t7699 = t7698 * t197;
    let t7700 = t7699 * t7346;
    let t7702 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t587 * t7700;
    let t7703 = t1885 * t1802;
    let t7704 = t1017 * t562;
    let t7705 = t7704 * t610;
    let t7706 = t7703 * t7705;
    let t7708 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1820 * t7706;
    (t7697, t7702, t7708)
}
