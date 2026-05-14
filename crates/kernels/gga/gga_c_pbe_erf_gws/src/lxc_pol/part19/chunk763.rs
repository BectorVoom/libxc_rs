//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 763/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk763<F: Float>(t197: F, t589: F, t172: F, t2824: F, t184: F, t2684: F, t5137: F, t639: F, t2571: F, t4934: F, t1620: F, t219: F, t2591: F, t649: F, t1639: F, t331: F) -> (F, F, F, F, F, F, F) {
    let t7148 = t589 * t197;
    let t7170 = t172 * t2824;
    let t7171 = t7170 * t184;
    let t7188 = t5137 * t2684;
    let t7190 = 16.0 / 135.0 * t639 * t7188;
    let t7191 = t4934 * t2571;
    let t7193 = 32.0 / 135.0 * t1620 * t7191;
    let t7194 = t2591 * t219;
    let t7199 = t2591 * t649;
    let t7209 = t331 * t1639;
    (t7148, t7171, t7190, t7193, t7194, t7199, t7209)
}
