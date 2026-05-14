//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1181/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1181<F: Float>(t21871: F, t4153: F, t11913: F, t6905: F, t2001: F, t5627: F, t1396: F, t4123: F, t1464: F, t4142: F, t6914: F, t3738: F, t6908: F, t1394: F, t556: F, t7052: F) -> (F, F, F, F, F, F, F) {
    let t21872 = t4153 * t21871;
    let t21874 = t11913 * t6905;
    let t21876 = t2001 * t5627;
    let t21877 = t1396 * t21876;
    let t21878 = t4123 * t21877;
    let t21879 = t1464 * t21878;
    let t21881 = t4142 * t6914;
    let t21883 = t3738 * t6908;
    let t21884 = t1394 * t21883;
    let t21886 = t7052 * t556;
    (t21872, t21874, t21876, t21879, t21881, t21884, t21886)
}
