//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 407/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk407(t3394: f64, t883: f64, t912: f64, t587: f64, t1445: f64, t3354: f64, t597: f64, t3194: f64, t2488: f64, t2487: f64, t2854: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3395 = t3394 * t883;
    let t3396 = t912 * t3395;
    let t3397 = t587 * t3396;
    let t3398 = 0.19171462976960374838e0_f64 * t3397;
    let t3399 = t1445 * t3354;
    let t3401 = 0.11502877786176224903e2_f64 * t597 * t3399;
    let t3406 = 0.15976219147466979032e-1_f64 * t3194;
    let t3407 = t2488 * t3395;
    let t3408 = t2487 * t3407;
    let t3409 = 0.19171462976960374838e0_f64 * t3408;
    let t3410 = t2854 * t874;
    (t3395, t3396, t3398, t3399, t3401, t3406, t3407, t3409, t3410)
}
