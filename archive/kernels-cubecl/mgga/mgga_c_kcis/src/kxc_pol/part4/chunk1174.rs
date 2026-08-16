//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1174/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1174<F: Float>(t14868: F, t5182: F, t1183: F, t3436: F, t1094: F, t5163: F, t1172: F, t10525: F, t284: F, t5048: F, t1175: F, t5042: F, sigma0: F) -> (F, F, F, F, F) {
    let t14869 = t14868 * t5182;
    let t14871 = t1183 * t3436;
    let t14872 = t14871 * t5182;
    let t14874 = t5163 * t1094;
    let t14875 = t14874 * sigma0;
    let t14876 = t14875 * t1172;
    let t14878 = t10525 * t284;
    let t14879 = t14878 * t5048;
    let t14881 = t1175 * t5042;
    (t14869, t14872, t14876, t14879, t14881)
}
