//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 720/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk720<F: Float>(t1489: F, t833: F, t4163: F, t4162: F, t4160: F, t552: F, t491: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t4164 = t833 * t1489;
    let t4165 = t4163 * t4164;
    let t4166 = t4162 * t4165;
    let t4167 = t4160 * t4166;
    let t4169 = t552 * sigma2;
    let t4170 = t4169 * t491;
    (t4164, t4165, t4166, t4167, t4169, t4170)
}
