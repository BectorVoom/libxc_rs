//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 865/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk865<F: Float>(t262: F, t75515: F, t7204: F, t14368: F, t15356: F, t15208: F, t70062: F, t14371: F, t15211: F, t15382: F, t1971: F, t495: F, t515: F, t8517: F) -> (F, F, F, F, F, F) {
    let t75516 = t262 * t75515;
    let t75517 = t7204 * t75516;
    let t75519 = t14368 * t15356;
    let t75522 = t70062 * t15208;
    let t75524 = t14371 * t15211;
    let t75531 = F::cast_from(0.23942587439980034662e-4_f64) * t8517 * t1971 * t515 * t15382 * t495;
    (t75516, t75517, t75519, t75522, t75524, t75531)
}
