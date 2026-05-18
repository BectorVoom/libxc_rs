//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 664/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk664<F: Float>(t10523: F, t1800: F, t1869: F, t4582: F, t4811: F, t4581: F, t5204: F, t4799: F, t1757: F, t4797: F, t1899: F, t1873: F) -> (F, F, F, F, F, F) {
    let t10524 = t1800 * t10523;
    let t10525 = t1869 * t10524;
    let t10527 = t4811 * t4582;
    let t10529 = t4581 * t5204;
    let t10530 = t1869 * t10529;
    let t10532 = t4811 * t4799;
    let t10534 = t4797 * t1757;
    let t10535 = t1899 * t10534;
    let t10536 = t1873 * t10535;
    (t10525, t10527, t10530, t10532, t10534, t10536)
}
