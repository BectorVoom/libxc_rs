//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 391/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk391<F: Float>(t3394: F, t883: F, t912: F, t587: F, t1445: F, t3354: F, t597: F, t3338: F, t569: F, t568: F, t3194: F, t2488: F, t2487: F, t2854: F, t874: F, t1562: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3395 = t3394 * t883;
    let t3396 = t912 * t3395;
    let t3397 = t587 * t3396;
    let t3398 = 0.19171462976960374838e0 * t3397;
    let t3399 = t1445 * t3354;
    let t3401 = 0.11502877786176224903e2 * t597 * t3399;
    let t3402 = t569 * t3338;
    let t3403 = t568 * t3402;
    let t3406 = 0.15976219147466979032e-1 * t3194;
    let t3407 = t2488 * t3395;
    let t3408 = t2487 * t3407;
    let t3409 = 0.19171462976960374838e0 * t3408;
    let t3410 = t2854 * t874;
    let t3411 = t1445 * t3410;
    let t3413 = 0.69017266717057349418e1 * t1562 * t3411;
    (t3395, t3396, t3398, t3399, t3401, t3402, t3403, t3406, t3407, t3409, t3410, t3411, t3413)
}
