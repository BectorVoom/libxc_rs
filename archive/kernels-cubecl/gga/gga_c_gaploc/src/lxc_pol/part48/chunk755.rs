//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 755/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk755<F: Float>(t1381: F, t3362: F, t1959: F, t3455: F, t10795: F, t747: F, t1: F, t10215: F, t106: F, t192: F, t10496: F, t540: F) -> (F, F, F, F, F) {
    let t33959 = t3362 * t1381;
    let t33992 = t3455 * t1959;
    let t34013 = t10795 * t747;
    let t34131 = t10215 * t1 * t106 * t192;
    let t34157 = t10496 * t540;
    (t33959, t33992, t34013, t34131, t34157)
}
