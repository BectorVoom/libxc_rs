//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 983/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk983<F: Float>(t7957: F, t7960: F, t1802: F, t3443: F, t610: F, t1885: F, t587: F, t1044: F, t7019: F, t7018: F, t1620: F, t1037: F, t7582: F) -> (F, F, F, F, F) {
    let t11108 = F::new(16.0) / F::new(135.0) * t7957;
    let t11109 = F::new(16.0) / F::new(45.0) * t7960;
    let t11110 = t1802 * t3443;
    let t11111 = t11110 * t610;
    let t11112 = t1885 * t11111;
    let t11114 = F::new(4.0) / F::new(15.0) * t587 * t11112;
    let t11115 = t7019 * t1044;
    let t11116 = t7018 * t11115;
    let t11118 = F::new(8.0) / F::new(15.0) * t1620 * t11116;
    let t11120 = F::new(8.0) / F::new(45.0) * t7582 * t1037;
    (t11108, t11109, t11114, t11118, t11120)
}
