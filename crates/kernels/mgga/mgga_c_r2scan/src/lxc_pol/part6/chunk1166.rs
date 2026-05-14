//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1166/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1166<F: Float>(t1399: F, t5772: F, t5599: F, t390: F, t5448: F, t5762: F, t652: F, t1647: F, t5381: F, t5388: F, t1376: F, t625: F, t1768: F, t5210: F, t5967: F, t1764: F) -> (F, F, F, F, F, F, F) {
    let t21270 = 0.28493333333333333333e0 * t1399 * t5772;
    let t21272 = 0.13746876075482378975e2 * t1399 * t5599;
    let t21276 = 0.41240628226447136925e2 * t390 * t5762 * t652 * t5448;
    let t21279 = 0.34737075717175875744e4 * t5388 * t5381 * t1647;
    let t21280 = t1376 * t625;
    let t21281 = t21280 * t1768;
    let t21283 = t5967 * t5210;
    let t21285 = t21280 * t1764;
    (t21270, t21272, t21276, t21279, t21281, t21283, t21285)
}
