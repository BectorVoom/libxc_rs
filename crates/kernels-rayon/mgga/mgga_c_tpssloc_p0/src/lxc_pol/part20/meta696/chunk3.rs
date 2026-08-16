//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2657/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2657(t16217: f64, t3866: f64, t1827: f64, t39947: f64, t16314: f64, t16398: f64, t16387: f64, t12251: f64, t12297: f64, t12351: f64, t12404: f64, t1363: f64, t16233: f64, t16278: f64, t16285: f64, t16394: f64, t3734: f64, t3853: f64, t40006: f64, t40008: f64, t40012: f64, t40019: f64, t40022: f64, t5187: f64, t5248: f64, t5249: f64, t820: f64) -> f64 {
    let t54191 = t3866 * t16217;
    let t54198 = t39947 * t1827;
    let t54199 = 119.0_f64 / 4608.0_f64 * t54198;
    let t54202 = t16398 * t16314;
    let t54213 = t16398 * t16387;
    let t54215 = -t16278 * t3853 / 1024.0_f64 + t16285 * t12297 / 512.0_f64 + 35.0_f64 / 64.0_f64 * t54191 - 15.0_f64 / 128.0_f64 * t1363 * t12351 * t820 * t5187 * t3734 - t54199 + t16394 * t12404 / 256.0_f64 + 7.0_f64 / 96.0_f64 * t54202 + 455.0_f64 / 216.0_f64 * t40006 - 35.0_f64 / 72.0_f64 * t40008 + 7.0_f64 / 144.0_f64 * t40012 + 35.0_f64 / 24.0_f64 * t40019 + 7.0_f64 / 12.0_f64 * t40022 - 3.0_f64 / 256.0_f64 * t16233 * t5248 * t5249 * t12251 - 7.0_f64 / 256.0_f64 * t54213;
    t54215
}
