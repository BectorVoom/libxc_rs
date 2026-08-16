//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 765/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk765<F: Float>(t9212: F, t591: F, t9: F, t21: F, t587: F, t14: F, t598: F, t2230: F, t594: F, t2229: F, t3: F, t19: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9213 = F::cast_from(0.4332e2_f64) * t9212;
    let t9214 = t9 * t591;
    let t9215 = F::cast_from(0.9288e2_f64) * t9214;
    let t9216 = t587 * t21;
    let t9217 = F::cast_from(0.3912e3_f64) * t9216;
    let t9218 = t14 * t598;
    let t9219 = F::cast_from(0.12804e4_f64) * t9218;
    let t9220 = t594 * t2230;
    let t9221 = F::cast_from(0.170856e4_f64) * t9220;
    let t9222 = t2229 * t3;
    let t9223 = F::cast_from(1.0_f64) / t9222;
    let t9225 = F::cast_from(0.75936e3_f64) * t19 * t9223;
    (t9213, t9214, t9215, t9216, t9217, t9218, t9219, t9221, t9223, t9225)
}
