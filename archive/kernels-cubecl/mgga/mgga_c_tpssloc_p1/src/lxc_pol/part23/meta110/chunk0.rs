//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 586/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk586<F: Float>(t1512: F, t2639: F, t157: F, t2658: F, t1409: F, t184: F, t1474: F, t172: F) -> (F, F, F, F) {
    let t4187 = t2639 * t1512;
    let t4194 = t2658 * t157;
    let t4195 = t184 * t1409;
    let t4199 = t1474 * t172;
    (t4187, t4194, t4195, t4199)
}
