//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 680/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk680<F: Float>(t4135: F, t4136: F, t1395: F, t1464: F, t1392: F, t2820: F, t86: F) -> (F, F, F, F) {
    let t4137 = t4135 * t4136;
    let t4138 = t1395 * t4137;
    let t4139 = t1464 * t4138;
    let t4142 = t86 * t2820 * t1392;
    (t4137, t4138, t4139, t4142)
}
