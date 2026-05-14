//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 992/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk992<F: Float>(t1864: F, t8858: F, t415: F, t15858: F, t6986: F, t5182: F, t15862: F, t6662: F, t16608: F, t22289: F, t5192: F, t6674: F, t15929: F, t15928: F, t22294: F, t6675: F) -> (F, F, F, F, F, F, F, F) {
    let t23012 = t1864 * t8858;
    let t23013 = t415 * t23012;
    let t23015 = t15858 * t6986;
    let t23016 = t5182 * t23015;
    let t23018 = t15862 * t6662;
    let t23019 = t5182 * t23018;
    let t23021 = t16608 * t22289;
    let t23022 = t5192 * t23021;
    let t23023 = t6674 * t23022;
    let t23025 = t15929 * t22289;
    let t23026 = t5192 * t23025;
    let t23027 = t15928 * t23026;
    let t23029 = t6675 * t22294;
    (t23013, t23016, t23019, t23021, t23023, t23025, t23027, t23029)
}
