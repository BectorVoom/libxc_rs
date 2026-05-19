//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 593/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk593<F: Float>(t3395: F, t912: F, t587: F, t1445: F, t3354: F, t597: F, t3194: F, t2488: F, t2487: F, t2854: F, t874: F, t1562: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3396 = t912 * t3395;
    let t3397 = t587 * t3396;
    let t3398 = F::cast_from(0.19171462976960374838e0_f64) * t3397;
    let t3399 = t1445 * t3354;
    let t3401 = F::cast_from(0.11502877786176224903e2_f64) * t597 * t3399;
    let t3406 = F::cast_from(0.15976219147466979032e-1_f64) * t3194;
    let t3407 = t2488 * t3395;
    let t3408 = t2487 * t3407;
    let t3409 = F::cast_from(0.19171462976960374838e0_f64) * t3408;
    let t3410 = t2854 * t874;
    let t3411 = t1445 * t3410;
    let t3413 = F::cast_from(0.69017266717057349418e1_f64) * t1562 * t3411;
    (t3396, t3398, t3399, t3401, t3406, t3407, t3409, t3410, t3411, t3413)
}
