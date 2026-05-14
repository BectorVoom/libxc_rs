//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 976/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk976<F: Float>(t12532: F, t7527: F, t3454: F, t16532: F, t185: F, t186: F, t40718: F, t1017: F, t1885: F, t40571: F, t587: F, t17182: F, t17183: F, t47391: F, t10383: F, t3443: F) -> (F, F, F, F, F, F) {
    let t47701 = 32.0 / 5.0 * t7527 * t12532;
    let t47702 = t3454 * t3454;
    let t47706 = 16.0 / 5.0 * t185 * t186 * t16532 * t47702;
    let t47707 = 32.0 / 27.0 * t40718;
    let t47711 = 16.0 / 15.0 * t587 * t1885 * t40571 * t1017;
    let t47715 = 352.0 / 243.0 * t587 * t17182 * t17183 * t47391;
    let t47719 = 24.0 / 5.0 * t587 * t1885 * t10383 * t3443;
    (t47701, t47706, t47707, t47711, t47715, t47719)
}
