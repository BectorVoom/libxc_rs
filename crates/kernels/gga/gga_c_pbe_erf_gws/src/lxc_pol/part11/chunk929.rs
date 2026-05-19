//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 929/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk929<F: Float>(t116: F, t366: F, t798: F, t799: F, t311: F, t19: F, t2331: F, t301: F, t305: F, t2082: F) -> (F, F, F) {
    let t19525 = F::cast_from(0.6693920255418271605e1_f64) * t798 * t799 * t366 * t116;
    let t19530 = t311 * t311;
    let t19537 = F::cast_from(0.34072858057724757727e0_f64) * t305 / t19530 * t2331 * t301 * t19 * t799;
    let t19560 = t2082 * t2082;
    let t19561 = F::new(1.0) / t19560;
    (t19525, t19537, t19561)
}
