//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 891/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk891<F: Float>(t1357: F, t16503: F, t34976: F, t8435: F, t10030: F, t34761: F, t1502: F, t571: F, t40771: F, t9147: F, t10066: F, t34764: F, t2298: F, t26370: F, t17859: F, t9051: F) -> (F, F, F, F, F, F, F) {
    let t47325 = t16503 * t34976 * t1357 * t8435;
    let t47327 = t34761 * t10030;
    let t47331 = t16503 * t34976 * t571 * t1502;
    let t47333 = t40771 * t9147;
    let t47335 = t34764 * t10066;
    let t47340 = t26370 * t2298;
    let t47345 = t17859 * t9051;
    (t47325, t47327, t47331, t47333, t47335, t47340, t47345)
}
