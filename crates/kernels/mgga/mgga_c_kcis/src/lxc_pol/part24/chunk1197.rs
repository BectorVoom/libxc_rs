//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1197/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1197<F: Float>(t27974: F, t7696: F, t26739: F, t27856: F, t1808: F, t982: F, t27910: F, t93435: F, t7703: F, t26685: F, t95890: F, t1250: F, t43526: F) -> (F, F, F, F, F, F, F) {
    let t96451 = F::cast_from(0.12356481481481481482e-2_f64) * t7696 * t27974;
    let t96456 = F::cast_from(0.16489724537037037037e-3_f64) * t26739 * t27856;
    let t96476 = t1808 * t982;
    let t96480 = t93435 * t27910;
    let t96482 = F::cast_from(0.46336805555555555556e-3_f64) * t7703 * t96480;
    let t96504 = t26685 * t96480;
    let t96508 = F::cast_from(0.18550940104166666667e-3_f64) * t26685 * t95890;
    let t96522 = t43526 * t1250;
    (t96451, t96456, t96476, t96482, t96504, t96508, t96522)
}
