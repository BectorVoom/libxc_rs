//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1064/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1064(t300: f64, t4865: f64, t4833: f64, t1687: f64, t1166: f64, t1703: f64, t3411: f64, t1694: f64, t3375: f64, t1157: f64, t1164: f64, t1147: f64, t1156: f64, t4857: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4866 = t300 * t4865;
    let t4868 = 0.19751673498613801407e-1_f64 * t300 * t4833;
    let t4869 = t300 * t1687;
    let t4871 = 0.5848223622634646207e0_f64 * t4869 * t1166;
    let t4873 = 0.5848223622634646207e0_f64 * t3411 * t1703;
    let t4874 = t3375 * t1694;
    let t4875 = t4874 * t1157;
    let t4877 = 0.11696447245269292414e1_f64 * t1164 * t4875;
    let t4879 = t1147 * t4857 * t1156;
    (t4866, t4868, t4869, t4871, t4873, t4874, t4875, t4877, t4879)
}
