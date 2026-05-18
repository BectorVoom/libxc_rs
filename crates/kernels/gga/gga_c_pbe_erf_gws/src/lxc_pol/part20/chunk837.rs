//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 837/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk837<F: Float>(t1336: F, t960: F, t1396: F, t2840: F, t1392: F, t1: F, t2474: F, t467: F, t1218: F, t75: F, t472: F, t4853: F) -> (F, F, F, F, F, F, F) {
    let t8014 = t1336 * t960;
    let t8016 = t2840 * t1396;
    let t8018 = t2840 * t1392;
    let t8020 = t2474 * t1;
    let t8021 = t8020 * t467;
    let t8022 = F::new(0.36623110073506319882e-3) * t8021;
    let t8023 = t2840 * t1218;
    let t8029 = t2474 * t75;
    let t8030 = t8029 * t472;
    let t8031 = F::new(0.11696446794910408142e1) * t8030;
    let t8033 = F::new(32.0) * t4853;
    (t8014, t8016, t8018, t8022, t8023, t8031, t8033)
}
