//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 885/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk885<F: Float>(t1620: F, t7653: F, t1821: F, t7359: F, t587: F, t1000: F, t1804: F, t5548: F, t2688: F, t5129: F, t2555: F, t5125: F) -> (F, F, F, F, F) {
    let t7655 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1620 * t7653;
    let t7656 = t1821 * t7359;
    let t7658 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t587 * t7656;
    let t7659 = t1000 * t1804;
    let t7660 = t5548 * t7659;
    let t7662 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t587 * t7660;
    let t7663 = t5129 * t2688;
    let t7665 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t587 * t7663;
    let t7666 = t5125 * t2555;
    (t7655, t7658, t7662, t7665, t7666)
}
