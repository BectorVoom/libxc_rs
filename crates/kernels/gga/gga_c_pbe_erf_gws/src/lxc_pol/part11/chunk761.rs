//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 761/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk761<F: Float>(t10025: F, t12335: F, t12336: F, t12337: F, t12363: F, t12364: F, t12365: F, t4652: F, t4664: F, t4744: F, t4751: F, t4754: F, t4784: F, t6076: F, t12366: F, t12369: F, t12370: F, t12372: F, t12373: F, t4688: F, t4711: F, t4714: F, t4718: F, t4790: F, t4799: F, t4803: F, t4807: F, t4815: F) -> (F, F) {
    let t13150 = 0.1232289865202e1 * t10025;
    let t13151 = t12335 + t12336 - t12337 + t4744 + t4751 + t4652 + t4754 + t12363 + t4664 - t6076 + t12364 - t12365 - t13150 - t4784;
    let t13153 = -t4790 + t12366 - t4799 - t4803 + t4807 - t4815 + t4688 + t4711 - t4714 - t4718 - t12369 + t12370 + t12372 - t12373;
    (t13151, t13153)
}
