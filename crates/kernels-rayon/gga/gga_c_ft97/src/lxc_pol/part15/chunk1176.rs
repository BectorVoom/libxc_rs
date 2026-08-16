//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1176/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1176(t5284: f64, t5295: f64, t1196: f64, t14722: f64, t83158: f64, t21999: f64, t88881: f64, t70474: f64, t83269: f64, t1197: f64, t1208: f64, t19080: f64, t19095: f64, t19233: f64, t22003: f64, t22082: f64, t22135: f64, t2691: f64, t4113: f64, t5016: f64, t5231: f64, t5265: f64, t5266: f64, t5285: f64, t5296: f64, t7003: f64, t70550: f64, t82845: f64, t83233: f64, t83313: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t89999 = t5284 * t5284;
    let t90003 = t5295 * t5295;
    let t90008 = t14722 * t83158 * t1196;
    let t90015 = t21999 * t88881;
    let t90049 = t70474 * t83269 * t1196;
    let t90054 = -12.0_f64 * t5231 * t5296 + 0.12383814134312858631e2_f64 * t7003 * t90015 - 12.0_f64 * t2691 * t19095 * t5295 + 0.1303559382559248277e1_f64 * t82845 * t1197 - 8.0_f64 * t2691 * t83313 * t1208 - 36.0_f64 * t4113 * t19080 * t5295 - 0.14498192132169191472e2_f64 * t83233 * t22003 - 0.35032929183548774393e2_f64 * t22082 * t5016 + 0.87582322958871935982e1_f64 * t5265 * t5266 * t5285 - 0.35032929183548774392e2_f64 * t70550 * t90049 + 8.0_f64 * t19233 * t22135;
    (t89999, t90003, t90008, t90015, t90049, t90054)
}
