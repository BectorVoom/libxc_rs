//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1004/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1004<F: Float>(t75340: F, t75344: F, t75347: F, t75352: F, t75360: F, t75375: F, t69294: F, t75386: F, t75388: F, t75390: F, t75393: F, t75395: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t78143 = F::cast_from(0.31062809106223861416e-1_f64) * t75340;
    let t78144 = F::cast_from(0.1814407727691612783e-2_f64) * t75344;
    let t78145 = F::cast_from(0.31752135234603223702e-2_f64) * t75347;
    let t78148 = F::cast_from(0.31752135234603223702e-2_f64) * t75352;
    let t78150 = F::cast_from(0.72324308034374009545e-3_f64) * t75360;
    let t78156 = F::cast_from(0.31062809106223861416e-2_f64) * t75375;
    let t78157 = F::cast_from(0.79828278012425390427e-1_f64) * t69294;
    let t78161 = F::cast_from(0.62125618212447722832e-2_f64) * t75386;
    let t78162 = F::cast_from(0.15531404553111930708e-1_f64) * t75388;
    let t78163 = F::cast_from(0.15531404553111930708e-1_f64) * t75390;
    let t78165 = F::cast_from(0.26609426004141796809e-1_f64) * t75393;
    let t78166 = F::cast_from(0.5987120850931904282e-1_f64) * t75395;
    (t78143, t78144, t78145, t78148, t78150, t78156, t78157, t78161, t78162, t78163, t78165, t78166)
}
