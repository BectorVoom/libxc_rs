//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 948/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk948<F: Float>(t7086: F, t7091: F, t7096: F, t7100: F, t7101: F, t7105: F, t7109: F, t7113: F, t7120: F, t7123: F, t7125: F, t7127: F, t7129: F, t7132: F, t7134: F, t7138: F) -> F {
    let t8418 = -t7086 - t7091 - t7096 + t7100 + t7101 + t7105 - t7109 - t7113 + t7120 + t7123 - t7125 - t7127 + t7129 + t7132 - t7134 + t7138;
    t8418
}
