//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 754/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk754<F: Float>(t10007: F, t10627: F, t32260: F, t739: F, t1: F, t106: F, t10667: F, t316: F, t1890: F, t32356: F, t11000: F, t783: F) -> (F, F, F, F, F) {
    let t33601 = t10007 * t10627;
    let t33676 = t739 * t32260;
    let t33725 = t10667 * t1 * t106 * t316;
    let t33760 = t1890 * t32356;
    let t33778 = t11000 * t783;
    (t33601, t33676, t33725, t33760, t33778)
}
