//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 971/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk971(t2487: f64, t2488: f64, t46401: f64, t11254: f64, t2464: f64, t2465: f64, t1445: f64, t41918: f64, t41954: f64, t41958: f64, t41960: f64, t41962: f64, t41981: f64, t44382: f64, t46368: f64, t46370: f64, t46371: f64, t46372: f64, t46379: f64, t46382: f64, t46384: f64, t46387: f64, t46390: f64, t46396: f64, t46398: f64, t597: f64) -> f64 {
    let t46403 = t2487 * t2488 * t46401;
    let t46404 = 0.19171462976960374838e0_f64 * t46403;
    let t46407 = t2487 * t2464 * t2465 * t11254;
    let t46408 = 0.42603251059911944084e-1_f64 * t46407;
    let t46409 = t46368 + 0.63904876589867916128e-1_f64 * t41918 - t46370 - t46371 - t46372 + 0.1022478025437886658e1_f64 * t41954 - 0.17875244975925213335e0_f64 * t41958 + 0.59584149919750711116e-1_f64 * t41960 + 0.59584149919750711116e-1_f64 * t41962 + t46379 + t46382 - 0.76685851907841499354e0_f64 * t46384 + 0.36425779656224712193e1_f64 * t46387 - 0.51762950037793012064e1_f64 * t46390 + 0.11502877786176224903e2_f64 * t597 * t1445 * t44382 - t41981 + t46396 + 0.76685851907841499353e0_f64 * t46398 + t46404 - t46408;
    t46409
}
