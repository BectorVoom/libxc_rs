//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 908/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk908<F: Float>(t11106: F, t11108: F, t11109: F, t11114: F, t11118: F, t11120: F, t11122: F, t11124: F, t11128: F, t11130: F, t11135: F, t11140: F, t11142: F, t11144: F, t11146: F, t7968: F, t7970: F) -> (F,) {
    let t11236 = t11106 - t11108 + t11109 + t11114 + t11118 + t11120 - t11122 + t11124 + t11128 + t11130 + t11135 - t11140 + t7968 + t7970 - t11142 - t11144 + t11146;
    (t11236,)
}
