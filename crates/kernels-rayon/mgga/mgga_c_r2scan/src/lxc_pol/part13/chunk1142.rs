//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1142/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1142(t10760: f64, t25997: f64, t6085: f64, t2147: f64, t25573: f64, t26997: f64, t11724: f64, t19883: f64, t37765: f64, t37770: f64, t37788: f64, t39686: f64, t39689: f64, t39692: f64, t39695: f64, t39697: f64) -> f64 {
    let t39700 = t6085 * t10760 * t25997;
    let t39703 = t2147 * t10760 * t25573;
    let t39706 = t6085 * t10760 * t26997;
    let t39708 = t19883 * t11724;
    let t39710 = -0.25610080155860322884e0_f64 * t37765 - 0.10975748638225852664e-1_f64 * t37770 - 0.23115257973478049502e0_f64 * t37788 + 0.65495539973149862688e-2_f64 * t39686 - 0.43663693315433241792e-2_f64 * t39689 - 0.13099107994629972538e-1_f64 * t39692 + 0.5239643197851989015e-1_f64 * t39695 - 0.43663693315433241792e-2_f64 * t39697 - 0.43663693315433241792e-2_f64 * t39700 + 0.43663693315433241792e-2_f64 * t39703 - 0.21831846657716620896e-2_f64 * t39706 - 0.43663693315433241792e-2_f64 * t39708;
    t39710
}
