//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1137/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1137<F: Float>(t16422: F, t3786: F, t3809: F, t5493: F, t1444: F, t1897: F, t2642: F, t3761: F, t1098: F, t5483: F, t1517: F, t531: F, t545: F, t1992: F, t3251: F, t16065: F, t5425: F) -> (F, F, F, F, F, F, F, F) {
    let t16423 = t3786 * t16422;
    let t16426 = t5493 * t3809;
    let t16427 = t3786 * t16426;
    let t16432 = t3761 * t1897 * t1444 * t2642;
    let t16436 = 0.19711289e-2 * t1098 * t5483;
    let t16438 = t1517 * t545 * t531;
    let t16441 = t3251 * t1992;
    let t16443 = t5425 * t16065;
    (t16423, t16426, t16427, t16432, t16436, t16438, t16441, t16443)
}
