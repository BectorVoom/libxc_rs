//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1142/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1142<F: Float>(t10760: F, t25997: F, t6085: F, t2147: F, t25573: F, t26997: F, t11724: F, t19883: F, t37765: F, t37770: F, t37788: F, t39686: F, t39689: F, t39692: F, t39695: F, t39697: F) -> F {
    let t39700 = t6085 * t10760 * t25997;
    let t39703 = t2147 * t10760 * t25573;
    let t39706 = t6085 * t10760 * t26997;
    let t39708 = t19883 * t11724;
    let t39710 = -F::cast_from(0.25610080155860322884e0_f64) * t37765 - F::cast_from(0.10975748638225852664e-1_f64) * t37770 - F::cast_from(0.23115257973478049502e0_f64) * t37788 + F::cast_from(0.65495539973149862688e-2_f64) * t39686 - F::cast_from(0.43663693315433241792e-2_f64) * t39689 - F::cast_from(0.13099107994629972538e-1_f64) * t39692 + F::cast_from(0.5239643197851989015e-1_f64) * t39695 - F::cast_from(0.43663693315433241792e-2_f64) * t39697 - F::cast_from(0.43663693315433241792e-2_f64) * t39700 + F::cast_from(0.43663693315433241792e-2_f64) * t39703 - F::cast_from(0.21831846657716620896e-2_f64) * t39706 - F::cast_from(0.43663693315433241792e-2_f64) * t39708;
    t39710
}
