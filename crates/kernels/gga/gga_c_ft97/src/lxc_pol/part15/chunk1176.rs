//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1176/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1176<F: Float>(t5284: F, t5295: F, t1196: F, t14722: F, t83158: F, t21999: F, t88881: F, t70474: F, t83269: F, t1197: F, t1208: F, t19080: F, t19095: F, t19233: F, t22003: F, t22082: F, t22135: F, t2691: F, t4113: F, t5016: F, t5231: F, t5265: F, t5266: F, t5285: F, t5296: F, t7003: F, t70550: F, t82845: F, t83233: F, t83313: F) -> (F, F, F, F, F, F) {
    let t89999 = t5284 * t5284;
    let t90003 = t5295 * t5295;
    let t90008 = t14722 * t83158 * t1196;
    let t90015 = t21999 * t88881;
    let t90049 = t70474 * t83269 * t1196;
    let t90054 = -F::cast_from(12.0_f64) * t5231 * t5296 + F::cast_from(0.12383814134312858631e2_f64) * t7003 * t90015 - F::cast_from(12.0_f64) * t2691 * t19095 * t5295 + F::cast_from(0.1303559382559248277e1_f64) * t82845 * t1197 - F::cast_from(8.0_f64) * t2691 * t83313 * t1208 - F::cast_from(36.0_f64) * t4113 * t19080 * t5295 - F::cast_from(0.14498192132169191472e2_f64) * t83233 * t22003 - F::cast_from(0.35032929183548774393e2_f64) * t22082 * t5016 + F::cast_from(0.87582322958871935982e1_f64) * t5265 * t5266 * t5285 - F::cast_from(0.35032929183548774392e2_f64) * t70550 * t90049 + F::cast_from(8.0_f64) * t19233 * t22135;
    (t89999, t90003, t90008, t90015, t90049, t90054)
}
