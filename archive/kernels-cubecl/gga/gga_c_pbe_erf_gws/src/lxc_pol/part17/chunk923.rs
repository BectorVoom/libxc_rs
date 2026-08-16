//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 923/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk923<F: Float>(t4351: F, t950: F, t1403: F, t1523: F, t34: F, t6937: F, t1407: F, t2477: F, t476: F, t532: F, t2480: F, t39: F) -> (F, F, F, F, F) {
    let t8078 = t4351 * t950;
    let t8079 = t8078 * t1403;
    let t8081 = t1523 * t34;
    let t8082 = t8081 * t6937;
    let t8084 = t2477 * t1407;
    let t8086 = t476 * t532;
    let t8088 = t2480 * t39;
    (t8079, t8082, t8084, t8086, t8088)
}
