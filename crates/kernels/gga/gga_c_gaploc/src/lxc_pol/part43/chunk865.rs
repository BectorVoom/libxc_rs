//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 865/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk865<F: Float>(t11977: F, t524: F, t3377: F, t13778: F, t2487: F, t6985: F, t1445: F, t46920: F, t597: F, t13813: F, t1562: F, t4614: F, t12078: F, t1415: F, t7030: F, t47953: F, t6716: F, t6717: F) -> (F, F, F, F, F, F) {
    let t48190 = t524 * t11977;
    let t48191 = t48190 * t3377;
    let t48194 = t2487 * t6985 * t13778;
    let t48198 = 0.11502877786176224903e2 * t597 * t1445 * t46920;
    let t48205 = t1562 * t4614 * t13813;
    let t48208 = t1415 * t12078 * t7030;
    let t48211 = t6716 * t6717 * t47953;
    (t48191, t48194, t48198, t48205, t48208, t48211)
}
