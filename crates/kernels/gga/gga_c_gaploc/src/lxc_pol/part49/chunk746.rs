//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 746/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk746<F: Float>(t11000: F, t783: F, t1381: F, t3362: F, t1959: F, t3455: F, t10795: F, t747: F, t1: F, t10215: F, t106: F, t192: F, t10496: F, t540: F, t1564: F, t10600: F, t1415: F) -> (F, F, F, F, F, F, F, F) {
    let t33778 = t11000 * t783;
    let t33959 = t3362 * t1381;
    let t33992 = t3455 * t1959;
    let t34013 = t10795 * t747;
    let t34131 = t10215 * t1 * t106 * t192;
    let t34157 = t10496 * t540;
    let t34202 = t1564 * t10215;
    let t34264 = t1415 * t10600;
    (t33778, t33959, t33992, t34013, t34131, t34157, t34202, t34264)
}
