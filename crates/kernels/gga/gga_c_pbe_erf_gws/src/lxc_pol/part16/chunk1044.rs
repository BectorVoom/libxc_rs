//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1044/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1044<F: Float>(t2100: F, t2118: F, t1105: F, t2079: F, t1112: F, t6469: F, t4408: F, t814: F, t6158: F, t6161: F, t2271: F, t810: F, t3037: F, t858: F, t892: F, t1114: F, t20112: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28647 = t2118 * t2100;
    let t28667 = t1105 * t2079;
    let t28672 = t6469 * t1112;
    let t28947 = t4408 * t814;
    let t29103 = t6158 * t6161;
    let t29117 = t2271 * t810;
    let t29287 = t2079 * t3037;
    let t29751 = t858 * t892;
    let t29775 = t1114 * t20112;
    (t28647, t28667, t28672, t28947, t29103, t29117, t29287, t29751, t29775)
}
