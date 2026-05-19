//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 234/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk234<F: Float>(t265: F, t735: F, t153: F, t274: F, t542: F, t168: F, t703: F) -> (F, F, F) {
    let t737 = F::new(2.0) / F::new(45.0) * t265 * t735;
    let t744 = F::cast_from(0.56945186695483624892e0_f64) * t153 * t542 * t274;
    let t751 = t168 * t703;
    (t737, t744, t751)
}
