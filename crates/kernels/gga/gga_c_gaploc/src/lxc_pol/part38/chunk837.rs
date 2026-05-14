//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 837/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk837<F: Float>(t11402: F, t2441: F, t13397: F, t21373: F, t6914: F, t11218: F, t123: F, t883: F, t2487: F, t2488: F, t11254: F, t2464: F, t2465: F, t1445: F, t41918: F, t41954: F, t41958: F, t41960: F, t41962: F, t41981: F, t44382: F, t46368: F, t46370: F, t46371: F, t46372: F, t46379: F, t46382: F, t46384: F, t46387: F, t46390: F, t597: F) -> (F, F) {
    let t46396 = 0.35750489951850426669e0 * t2441 * t11402;
    let t46398 = t6914 * t21373 * t13397;
    let t46401 = t11218 * t123 * t883;
    let t46403 = t2487 * t2488 * t46401;
    let t46404 = 0.19171462976960374838e0 * t46403;
    let t46407 = t2487 * t2464 * t2465 * t11254;
    let t46408 = 0.42603251059911944084e-1 * t46407;
    let t46409 = t46368 + 0.63904876589867916128e-1 * t41918 - t46370 - t46371 - t46372 + 0.1022478025437886658e1 * t41954 - 0.17875244975925213335e0 * t41958 + 0.59584149919750711116e-1 * t41960 + 0.59584149919750711116e-1 * t41962 + t46379 + t46382 - 0.76685851907841499354e0 * t46384 + 0.36425779656224712193e1 * t46387 - 0.51762950037793012064e1 * t46390 + 0.11502877786176224903e2 * t597 * t1445 * t44382 - t41981 + t46396 + 0.76685851907841499353e0 * t46398 + t46404 - t46408;
    (t46401, t46409)
}
