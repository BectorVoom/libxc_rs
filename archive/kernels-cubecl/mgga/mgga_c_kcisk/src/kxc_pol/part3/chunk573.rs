//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 573/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk573<F: Float>(t1801: F, t4797: F, t1873: F, t1869: F, t3293: F, t3499: F, t3500: F, t8: F) -> (F, F, F, F) {
    let t4798 = t1801 * t4797;
    let t4799 = t1873 * t4798;
    let t4800 = t1869 * t4799;
    let t4803 = t3293 * t8 - t3499 + t3500;
    (t4798, t4799, t4800, t4803)
}
