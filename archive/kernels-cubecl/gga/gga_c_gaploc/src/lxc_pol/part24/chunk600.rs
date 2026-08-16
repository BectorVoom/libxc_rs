//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 600/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk600<F: Float>(t3395: F, t912: F, t587: F, t1445: F, t3354: F, t597: F, t3338: F, t569: F, t568: F, t3194: F, t2488: F, t2487: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3396 = t912 * t3395;
    let t3397 = t587 * t3396;
    let t3398 = F::cast_from(0.19171462976960374838e0_f64) * t3397;
    let t3399 = t1445 * t3354;
    let t3401 = F::cast_from(0.11502877786176224903e2_f64) * t597 * t3399;
    let t3402 = t569 * t3338;
    let t3403 = t568 * t3402;
    let t3406 = F::cast_from(0.15976219147466979032e-1_f64) * t3194;
    let t3407 = t2488 * t3395;
    let t3408 = t2487 * t3407;
    (t3396, t3398, t3399, t3401, t3402, t3403, t3406, t3407, t3408)
}
