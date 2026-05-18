//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1008/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1008<F: Float>(t4688: F, t4711: F, t4714: F, t4718: F, t4799: F, t4803: F, t4807: F, t4815: F, t8002: F, t8003: F, t8005: F, t8008: F, t8009: F, t8011: F, t8013: F, t8015: F, t8017: F, t8019: F) -> F {
    let t9046 = -t8002 + t8003 + t8005 - t4799 - t4803 + t4807 + t8008 - t4815 + t4688 + t4711 - t4714 - t4718 - t8009 + t8011 - t8013 + t8015 - t8017 - t8019;
    t9046
}
