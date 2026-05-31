//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 866/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk866<F: Float>(t5323: F, t1809: F, t7257: F, t639: F, t1027: F, t1793: F, t4927: F, t2559: F, t7336: F, t587: F, t197: F, t5293: F) -> (F, F, F, F, F) {
    let t7424 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t5323;
    let t7425 = t1809 * t7257;
    let t7427 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t639 * t7425;
    let t7428 = t1027 * t1793;
    let t7429 = t4927 * t7428;
    let t7431 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t639 * t7429;
    let t7432 = t2559 * t7336;
    let t7434 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t587 * t7432;
    let t7435 = t5293 * t197;
    (t7424, t7427, t7431, t7434, t7435)
}
