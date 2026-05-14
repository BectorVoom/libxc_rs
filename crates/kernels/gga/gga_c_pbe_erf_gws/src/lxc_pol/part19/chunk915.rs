//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 915/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk915<F: Float>(t10251: F, t10254: F, t10255: F, t10256: F, t10258: F, t10260: F, t10261: F, t10262: F, t4688: F, t4711: F, t4714: F, t4718: F, t4799: F, t4803: F, t4807: F, t4815: F, t8011: F, t8022: F) -> (F,) {
    let t11312 = t10251 - t4799 - t4803 + t4807 + t10254 - t4815 + t4688 + t4711 - t4714 - t4718 - t8011 - t10255 - t10256 + t10258 - t10260 - t10261 - t10262 - t8022;
    (t11312,)
}
