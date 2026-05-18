//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 873/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk873<F: Float>(t1828: F, t7495: F, t5218: F, t5212: F, t626: F, t661: F, t954: F, t617: F, t5211: F, t1697: F, t422: F, t7115: F) -> (F, F, F, F) {
    let t7496 = t7495 * t1828;
    let t7498 = F::new(16.0) / F::new(45.0) * t5218 * t7496;
    let t7499 = t5212 * t626;
    let t7500 = t954 * t661;
    let t7502 = t7499 * t7500 * t617;
    let t7504 = F::new(16.0) / F::new(45.0) * t5211 * t7502;
    let t7505 = t5212 * t1697;
    let t7506 = t7500 * t422;
    let t7507 = t7505 * t7506;
    let t7509 = F::new(16.0) / F::new(45.0) * t7115 * t7507;
    (t7498, t7504, t7506, t7509)
}
