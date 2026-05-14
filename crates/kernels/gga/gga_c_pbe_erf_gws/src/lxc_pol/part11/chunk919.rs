//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 919/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk919<F: Float>(t12955: F, t395: F, t501: F, t12930: F, t156: F, t496: F, t42658: F, t8135: F, t12937: F, t12929: F, t1563: F, t12962: F, t513: F, t12978: F, t2911: F, t8236: F) -> (F, F, F, F, F, F, F, F) {
    let t42678 = t501 * t12955 * t395;
    let t42680 = t156 * t12930;
    let t42681 = t496 * t42680;
    let t42683 = t8135 * t42658;
    let t42714 = t496 * t156 * t12937;
    let t42719 = t1563 * t12929;
    let t42742 = t12962 * t513;
    let t42806 = t2911 * t8236 * t12978;
    (t42678, t42680, t42681, t42683, t42714, t42719, t42742, t42806)
}
