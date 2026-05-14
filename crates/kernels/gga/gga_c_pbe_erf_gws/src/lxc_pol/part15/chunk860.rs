//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 860/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk860<F: Float>(t20: F, t2653: F, t2004: F, t5919: F, t5922: F, t7179: F, t7180: F, t7184: F, t7185: F, t7187: F, t7190: F, t7193: F, t7198: F, t7203: F, t7208: F, t7215: F, t7221: F, t7223: F) -> (F,) {
    let t8424 = t2653 * t20;
    let t8425 = t8424 * t2004;
    let t8427 = -t7179 - t7180 + t7184 + t7185 + t7187 + 0.11181742741110338156e-1 * t8425 - t5919 + t5922 - t7190 + t7193 - t7198 + t7203 + t7208 - t7215 + t7221 + t7223;
    (t8427,)
}
