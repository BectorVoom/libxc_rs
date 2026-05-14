//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1024/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1024<F: Float>(t11724: F, t19883: F, t37765: F, t37770: F, t37788: F, t39686: F, t39689: F, t39692: F, t39695: F, t39697: F, t39700: F, t39703: F, t39706: F, t11675: F, t19872: F, t11678: F, t6395: F) -> (F, F, F) {
    let t39708 = t19883 * t11724;
    let t39710 = -0.25610080155860322884e0 * t37765 - 0.10975748638225852664e-1 * t37770 - 0.23115257973478049502e0 * t37788 + 0.65495539973149862688e-2 * t39686 - 0.43663693315433241792e-2 * t39689 - 0.13099107994629972538e-1 * t39692 + 0.5239643197851989015e-1 * t39695 - 0.43663693315433241792e-2 * t39697 - 0.43663693315433241792e-2 * t39700 + 0.43663693315433241792e-2 * t39703 - 0.21831846657716620896e-2 * t39706 - 0.43663693315433241792e-2 * t39708;
    let t39713 = t19872 * t11675;
    let t39715 = t6395 * t11678;
    (t39710, t39713, t39715)
}
