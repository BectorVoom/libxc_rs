//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1007/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1007<F: Float>(t14498: F, t3249: F, t3299: F, t4039: F, t1154: F, t14079: F, t3172: F, t4028: F, t3184: F, t14101: F, t3142: F, t3148: F, t3279: F, t4049: F, t14011: F, t3232: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14499 = t14498 * t3249;
    let t14502 = t4039 * t3299;
    let t14506 = t14079 * t1154;
    let t14508 = t4028 * t3172;
    let t14510 = t4028 * t3184;
    let t14512 = t14101 * t3142;
    let t14514 = t4028 * t3148;
    let t14516 = t4049 * t3279;
    let t14518 = t14011 * t3232;
    (t14499, t14502, t14506, t14508, t14510, t14512, t14514, t14516, t14518)
}
