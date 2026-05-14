//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 890/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk890<F: Float>(t2562: F, t2615: F, t11108: F, t11109: F, t11114: F, t11118: F, t11120: F, t11122: F, t11124: F, t11128: F, t11130: F, t11135: F, t11140: F, t11142: F, t11144: F, t5562: F, t7968: F, t7970: F) -> (F, F) {
    let t11146 = 8.0 / 27.0 * t2615 * t2562;
    let t11147 = t5562 - t11108 + t11109 + t11114 + t11118 + t11120 - t11122 + t11124 + t11128 + t11130 + t11135 - t11140 + t7968 + t7970 - t11142 - t11144 + t11146;
    (t11146, t11147)
}
