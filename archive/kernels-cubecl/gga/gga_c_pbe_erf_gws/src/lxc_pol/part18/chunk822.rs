//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 822/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk822<F: Float>(t1014: F, t1251: F, t2781: F, t7236: F, t1006: F, t1673: F, t197: F, t5293: F, t1036: F, t5463: F, t639: F, t188: F, t331: F) -> (F, F, F, F, F, F) {
    let t7407 = t1251 * t1014;
    let t7409 = t7236 * t2781;
    let t7421 = t1006 * t1673;
    let t7435 = t5293 * t197;
    let t7459 = t5463 * t1036;
    let t7460 = t639 * t7459;
    let t7467 = t331 * t188;
    (t7407, t7409, t7421, t7435, t7460, t7467)
}
