//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 825/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk825<F: Float>(t12366: F, t12369: F, t12370: F, t12372: F, t12373: F, t4688: F, t4711: F, t4714: F, t4718: F, t4790: F, t4799: F, t4803: F, t4807: F, t4815: F) -> F {
    let t13153 = -t4790 + t12366 - t4799 - t4803 + t4807 - t4815 + t4688 + t4711 - t4714 - t4718 - t12369 + t12370 + t12372 - t12373;
    t13153
}
