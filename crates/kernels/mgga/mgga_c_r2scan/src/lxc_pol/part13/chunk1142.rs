//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1142/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1142<F: Float>(t10760: F, t25997: F, t6085: F, t2147: F, t25573: F, t26997: F, t11724: F, t19883: F, t37765: F, t37770: F, t37788: F, t39686: F, t39689: F, t39692: F, t39695: F, t39697: F) -> F {
    let t39700 = t6085 * t10760 * t25997;
    let t39703 = t2147 * t10760 * t25573;
    let t39706 = t6085 * t10760 * t26997;
    let t39708 = t19883 * t11724;
    let t39710 = -F::new(0.25610080155860322884e0) * t37765 - F::new(0.10975748638225852664e-1) * t37770 - F::new(0.23115257973478049502e0) * t37788 + F::new(0.65495539973149862688e-2) * t39686 - F::new(0.43663693315433241792e-2) * t39689 - F::new(0.13099107994629972538e-1) * t39692 + F::new(0.5239643197851989015e-1) * t39695 - F::new(0.43663693315433241792e-2) * t39697 - F::new(0.43663693315433241792e-2) * t39700 + F::new(0.43663693315433241792e-2) * t39703 - F::new(0.21831846657716620896e-2) * t39706 - F::new(0.43663693315433241792e-2) * t39708;
    t39710
}
