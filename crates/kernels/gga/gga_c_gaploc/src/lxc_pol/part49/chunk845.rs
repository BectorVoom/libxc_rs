//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 845/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk845<F: Float>(t1: F, t106: F, t10667: F, t316: F, t11000: F, t783: F, t1381: F, t3362: F, t1959: F, t3455: F, t10795: F, t747: F) -> (F, F, F, F, F) {
    let t33725 = t10667 * t1 * t106 * t316;
    let t33778 = t11000 * t783;
    let t33959 = t3362 * t1381;
    let t33992 = t3455 * t1959;
    let t34013 = t10795 * t747;
    (t33725, t33778, t33959, t33992, t34013)
}
