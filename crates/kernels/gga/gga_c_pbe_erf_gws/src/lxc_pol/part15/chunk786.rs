//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 786/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk786<F: Float>(t5169: F, t5172: F, t5208: F, t2684: F, t5137: F, t639: F, t2571: F, t4934: F, t1620: F, t219: F, t2591: F, t2705: F, t617: F, t649: F, t661: F, t1697: F, t34: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7184 = 16.0 / 45.0 * t5169;
    let t7185 = 8.0 / 45.0 * t5172;
    let t7187 = 8.0 / 135.0 * t5208;
    let t7188 = t5137 * t2684;
    let t7190 = 16.0 / 135.0 * t639 * t7188;
    let t7191 = t4934 * t2571;
    let t7193 = 32.0 / 135.0 * t1620 * t7191;
    let t7194 = t2591 * t219;
    let t7195 = t2705 * t617;
    let t7196 = t7194 * t7195;
    let t7198 = 32.0 / 45.0 * t1620 * t7196;
    let t7199 = t2591 * t649;
    let t7200 = t2705 * t661;
    let t7201 = t7199 * t7200;
    let t7203 = 16.0 / 45.0 * t639 * t7201;
    let t7204 = t1697 * t34;
    (t7184, t7185, t7187, t7190, t7193, t7194, t7198, t7203, t7204)
}
