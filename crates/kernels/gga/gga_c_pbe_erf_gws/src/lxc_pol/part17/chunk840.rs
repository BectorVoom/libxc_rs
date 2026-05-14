//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 840/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk840<F: Float>(t6952: F, t8093: F, t1416: F, t2485: F, t478: F, t532: F, t2488: F, t39: F, t8079: F, t8082: F, t8084: F, t8086: F, t8088: F, t8091: F, t142: F, t2873: F) -> (F, F, F, F, F, F) {
    let t8094 = t8093 * t6952;
    let t8096 = t2485 * t1416;
    let t8098 = t478 * t532;
    let t8100 = t2488 * t39;
    let t8102 = 4.0 / 27.0 * t8079 - 4.0 / 9.0 * t8082 - t8084 / 9.0 + 2.0 / 3.0 * t8086 - 2.0 * t8088 + 4.0 / 27.0 * t8091 + 4.0 / 9.0 * t8094 - t8096 / 9.0 - 2.0 / 3.0 * t8098 + 2.0 * t8100;
    let t8108 = t142 * t2873;
    (t8094, t8096, t8098, t8100, t8102, t8108)
}
