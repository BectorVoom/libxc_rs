//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1391/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1391<F: Float>(t16793: F, t16804: F, t16806: F, t16808: F, t2096: F, t4422: F, t16073: F, t6159: F, t5713: F, t617: F, t16078: F, t16060: F, t6151: F) -> (F, F, F, F, F, F, F, F) {
    let t18056 = F::cast_from(0.15476481481481481481e-2_f64) * t16793;
    let t18059 = F::cast_from(0.23214722222222222222e-2_f64) * t16804;
    let t18060 = F::cast_from(0.15476481481481481481e-2_f64) * t16806;
    let t18061 = F::cast_from(0.15476481481481481481e-2_f64) * t16808;
    let t18069 = t2096 * t4422;
    let t18071 = t6159 * t16073;
    let t18079 = t5713 * t617;
    let t18080 = t18079 * t16078;
    let t18083 = t6151 * t16060;
    (t18056, t18059, t18060, t18061, t18069, t18071, t18080, t18083)
}
