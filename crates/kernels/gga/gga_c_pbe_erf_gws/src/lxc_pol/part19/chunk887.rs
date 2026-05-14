//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 887/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk887<F: Float>(t7957: F, t7960: F, t1802: F, t3443: F, t610: F, t1885: F, t587: F, t1044: F, t7019: F, t7018: F, t1620: F, t1037: F, t7582: F, t2593: F, t2612: F, t1627: F, t3519: F) -> (F, F, F, F, F, F, F) {
    let t11108 = 16.0 / 135.0 * t7957;
    let t11109 = 16.0 / 45.0 * t7960;
    let t11110 = t1802 * t3443;
    let t11111 = t11110 * t610;
    let t11112 = t1885 * t11111;
    let t11114 = 4.0 / 15.0 * t587 * t11112;
    let t11115 = t7019 * t1044;
    let t11116 = t7018 * t11115;
    let t11118 = 8.0 / 15.0 * t1620 * t11116;
    let t11120 = 8.0 / 45.0 * t7582 * t1037;
    let t11122 = 16.0 / 45.0 * t2612 * t2593;
    let t11124 = 4.0 / 45.0 * t1627 * t3519;
    (t11108, t11109, t11114, t11118, t11120, t11122, t11124)
}
