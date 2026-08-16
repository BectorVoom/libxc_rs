//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1046/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1046<F: Float>(t12167: F, t550: F, t1380: F, t1372: F, t3787: F, t3793: F, t1351: F, t3791: F, t3856: F, t3901: F, t215: F, t535: F, t9569: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12168 = t12167 * t550;
    let t12169 = t1380 * t12168;
    let t12171 = t3787 * t1372;
    let t12172 = t12171 * t3793;
    let t12177 = t3791 * t1351;
    let t12178 = t12177 * t550;
    let t12179 = t1380 * t12178;
    let t12181 = t3901 * t3856;
    let t12188 = F::cast_from(0.28086419753086419752e-1_f64) * t9569 * t535 * t215;
    (t12168, t12169, t12171, t12172, t12177, t12178, t12179, t12181, t12188)
}
