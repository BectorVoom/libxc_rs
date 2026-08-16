//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 689/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk689<F: Float>(t1338: F, t68: F, t544: F, t2235: F, t33: F, t645: F, t79: F, t72: F, t605: F, t608: F, t625: F, t641: F, t71: F) -> (F, F, F, F, F, F) {
    let t5343 = t68 * t1338;
    let t5344 = t544 * t5343;
    let t6486 = t2235 * t33;
    let t6491 = t79 * t645;
    let t6492 = t72 * t6491;
    let t6495 = t605 * t608;
    let t6503 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t625;
    let t6509 = t71 * t641;
    (t5344, t6486, t6492, t6495, t6503, t6509)
}
