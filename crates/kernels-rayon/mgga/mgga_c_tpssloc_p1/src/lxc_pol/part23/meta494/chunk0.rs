//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1519/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1519(t12291: f64, t1341: f64, t1343: f64, t16285: f64, t1827: f64, t19855: f64, t20492: f64, t20497: f64, t20556: f64, t20570: f64, t3790: f64, t40449: f64, t5235: f64, t54020: f64, t54793: f64, t6417: f64, t6422: f64, t74290: f64, t80076: f64, t80085: f64, t80189: f64, t80193: f64, t820: f64) -> f64 {
    let t80474 = -t1341 * t1343 * t820 * t80193 / 3072.0_f64 - t74290 * t1827 / 768.0_f64 - t19855 * t6417 / 512.0_f64 - t5235 * t20556 / 768.0_f64 + t16285 * t20497 / 128.0_f64 - t19855 * t6422 / 512.0_f64 - 3.0_f64 / 256.0_f64 * t12291 * t1343 * t820 * t80189 - t5235 * t20570 / 768.0_f64 - t54020 * t20492 / 128.0_f64 - t1341 * t1343 * t820 * t80076 / 1024.0_f64 - 595.0_f64 / 2592.0_f64 * t54793 + t40449 + t3790 * t1343 * t820 * t80085 / 512.0_f64;
    t80474
}
