//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 401/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk401<F: Float>(t3487: F, t883: F, t969: F, t825: F, t1445: F, t3447: F, t833: F, t3431: F, t808: F, t568: F, t3309: F, t2685: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3488 = t3487 * t883;
    let t3489 = t969 * t3488;
    let t3490 = t825 * t3489;
    let t3491 = F::cast_from(0.19171462976960374838e0_f64) * t3490;
    let t3492 = t1445 * t3447;
    let t3494 = F::cast_from(0.11502877786176224903e2_f64) * t833 * t3492;
    let t3495 = t808 * t3431;
    let t3496 = t568 * t3495;
    let t3499 = F::cast_from(0.15976219147466979032e-1_f64) * t3309;
    let t3500 = t2685 * t3488;
    (t3488, t3489, t3490, t3491, t3492, t3494, t3495, t3496, t3499, t3500)
}
