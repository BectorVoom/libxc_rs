//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1229/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1229<F: Float>(t1468: F, t15861: F, t1464: F, t1495: F, t4169: F, t4172: F, t5440: F, t4160: F, t5426: F, t5661: F, t1889: F, t4129: F) -> (F, F, F, F) {
    let t15862 = t1468 * t15861;
    let t15863 = t1464 * t15862;
    let t15865 = t4169 * t1495;
    let t15866 = t5440 * t4172;
    let t15867 = t15865 * t15866;
    let t15868 = t4160 * t15867;
    let t15870 = t5426 * t4172;
    let t15871 = t15865 * t15870;
    let t15872 = t5661 * t15871;
    let t15874 = t1889 * t4129;
    (t15863, t15868, t15872, t15874)
}
