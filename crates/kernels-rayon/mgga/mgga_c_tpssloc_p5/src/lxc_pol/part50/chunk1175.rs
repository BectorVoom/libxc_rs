//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1175/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1175(t30714: f64, t4191: f64, t112818: f64, t112820: f64, t112829: f64, t112835: f64, t112841: f64, t112846: f64, t112851: f64, t112856: f64, t118586: f64, t118588: f64, t118590: f64, t118592: f64, t118594: f64, t118596: f64, t118602: f64, t118606: f64, t118608: f64, t118610: f64) -> f64 {
    let t118612 = t30714 * t4191;
    let t118615 = 0.13457585364713463618e-3_f64 * t118586 + 7.0_f64 / 576.0_f64 * t118588 - t118590 / 384.0_f64 - t118592 / 384.0_f64 - t118594 / 384.0_f64 + 7.0_f64 / 2304.0_f64 * t118596 + 0.80745512188280781708e-3_f64 * t112818 + 7.0_f64 / 576.0_f64 * t112820 + 0.56521858531796547196e-2_f64 * t112829 - 7.0_f64 / 2304.0_f64 * t118602 + t112835 - t112841 - 0.48447307312968469025e-2_f64 * t118606 - t118608 / 1536.0_f64 + t118610 / 384.0_f64 + t118612 / 384.0_f64 - 7.0_f64 / 2304.0_f64 * t112846 + t112851 + t112856;
    t118615
}
