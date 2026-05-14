//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 814/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk814<F: Float>(t4135: F, t5875: F, t1395: F, t1464: F, t1497: F, t2001: F) -> (F, F, F, F) {
    let t5876 = t4135 * t5875;
    let t5877 = t1395 * t5876;
    let t5878 = t1464 * t5877;
    let t5880 = t2001 * t1497;
    (t5876, t5877, t5878, t5880)
}
