//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 850/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk850<F: Float>(t14173: F, t1652: F, t1550: F, t16503: F, t35039: F, t665: F, t8420: F, t16504: F, t8425: F, t3369: F, t8430: F, t2024: F, t34976: F, t8435: F) -> (F, F, F, F, F, F) {
    let t75216 = t14173 * t1652;
    let t75217 = t1550 * t75216;
    let t75221 = t16503 * t35039 * t665 * t8420;
    let t75225 = t16503 * t16504 * t665 * t8425;
    let t75231 = t16503 * t3369 * t665 * t8430;
    let t75235 = t16503 * t34976 * t2024 * t8435;
    (t75216, t75217, t75221, t75225, t75231, t75235)
}
