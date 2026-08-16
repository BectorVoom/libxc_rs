//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 925/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk925<F: Float>(t4366: F, t954: F, t1413: F, t1528: F, t34: F, t6952: F, t1416: F, t2485: F, t478: F, t532: F, t2488: F, t39: F) -> (F, F, F, F, F) {
    let t8090 = t4366 * t954;
    let t8091 = t8090 * t1413;
    let t8093 = t1528 * t34;
    let t8094 = t8093 * t6952;
    let t8096 = t2485 * t1416;
    let t8098 = t478 * t532;
    let t8100 = t2488 * t39;
    (t8091, t8094, t8096, t8098, t8100)
}
