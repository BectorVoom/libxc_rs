//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1064/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1064<F: Float>(t1318: F, t3899: F, t7569: F, t4763: F, t6894: F, t2143: F, t6198: F, t6190: F, t20027: F, t571: F, t574: F, t575: F, t1472: F, t7613: F, t7609: F, t1466: F, t2065: F, t6968: F) -> (F, F, F, F, F, F, F, F) {
    let t22156 = t1318 * t3899 * t7569;
    let t22157 = 8.0 / 15.0 * t22156;
    let t22158 = t4763 * t6894;
    let t22159 = 16.0 / 15.0 * t22158;
    let t22160 = t6198 * t2143;
    let t22161 = 8.0 / 45.0 * t22160;
    let t22163 = 4.0 / 5.0 * t4763 * t6190;
    let t22167 = 4.0 / 45.0 * t571 * t574 * t575 * t20027;
    let t22169 = 32.0 / 81.0 * t1472 * t7613;
    let t22171 = 4.0 / 45.0 * t1472 * t7609;
    let t22175 = 12.0 / 5.0 * t571 * t1466 * t6968 * t2065;
    (t22157, t22159, t22161, t22163, t22167, t22169, t22171, t22175)
}
