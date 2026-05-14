//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 777/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk777<F: Float>(t1: F, t2474: F, t467: F, t1218: F, t2840: F, t75: F, t472: F, t4853: F, t4857: F, t4860: F, t242: F, t3013: F, t153: F, t2848: F, t542: F, t145: F, t2522: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8020 = t2474 * t1;
    let t8021 = t8020 * t467;
    let t8022 = 0.36623110073506319882e-3 * t8021;
    let t8023 = t2840 * t1218;
    let t8029 = t2474 * t75;
    let t8030 = t8029 * t472;
    let t8031 = 0.11696446794910408142e1 * t8030;
    let t8033 = 32.0 * t4853;
    let t8034 = 48.0 * t4857;
    let t8035 = 80.0 * t4860;
    let t8042 = t3013 * t242;
    let t8047 = 0.11389037339096724978e1 * t153 * t542 * t2848;
    let t8048 = t145 * t2522;
    (t8022, t8023, t8031, t8033, t8034, t8035, t8042, t8047, t8048)
}
