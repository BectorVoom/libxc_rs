//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1117/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1117<F: Float>(t14498: F, t9671: F, t14028: F, t3299: F, t14567: F, t6608: F, t9484: F, t14535: F, t2115: F, t14538: F, t51282: F, t2129: F, t51306: F, t9500: F, t51351: F, t9389: F) -> (F, F, F, F, F, F, F, F) {
    let t54196 = t14498 * t9671;
    let t54198 = t14028 * t3299;
    let t54201 = t6608 * t9484 * t14567;
    let t54203 = t2115 * t14535;
    let t54205 = t14538 * t51282;
    let t54207 = t2129 * t14535;
    let t54209 = t51306 * t9500;
    let t54215 = t51351 * t9389;
    (t54196, t54198, t54201, t54203, t54205, t54207, t54209, t54215)
}
