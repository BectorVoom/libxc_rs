//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1356/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1356(t204: f64, t376: f64, t1020: f64, t1023: f64, t248: f64, t10510: f64, t3109: f64, t10309: f64, t10390: f64, t10398: f64, t10408: f64, t10410: f64, t10413: f64, t10419: f64, t10493: f64, t10858: f64, t10886: f64, t10937: f64, t2776: f64, t3041: f64, t3070: f64, t3071: f64, t3117: f64, t43186: f64, t43200: f64, t43206: f64, t43211: f64, t43214: f64, t884: f64) -> f64 {
    let t43216 = t204 * t376;
    let t43219 = t1020 * t248 * t43216 * t1023;
    let t43221 = t3109 * t10510;
    let t43223 = t3070 * t3071 * t10858 * t884 / 1152.0_f64 + t43186 / 288.0_f64 - t10390 * t10419 / 192.0_f64 - 5.0_f64 / 576.0_f64 * t3070 * t10408 * t10309 * t1023 + t10413 * t3071 * t3041 * t2776 / 384.0_f64 - t43200 / 1728.0_f64 - t10937 * t10398 / 72.0_f64 - 5.0_f64 / 216.0_f64 * t10937 * t10410 - t43206 / 288.0_f64 + t3117 * t10493 / 192.0_f64 - t43211 * t10886 / 144.0_f64 + t43214 / 324.0_f64 + t43219 / 2592.0_f64 + t43221 / 216.0_f64;
    t43223
}
