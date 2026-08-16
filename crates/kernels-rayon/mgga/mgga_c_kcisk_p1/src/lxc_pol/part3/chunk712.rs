//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 712/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk712(t11051: f64, t1714: f64, t1707: f64, t606: f64, t11037: f64, t1709: f64, t4873: f64, t4881: f64, t4864: f64, t10944: f64, t10947: f64, t10951: f64, t10954: f64, t10960: f64, t10966: f64, t11030: f64, t11033: f64, t11038: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11052 = t1714 * t11051;
    let t11054 = t1707 * t11051;
    let t11056 = 1.0_f64/pow_3_2(t606);
    let t11057 = t11056 * t11037;
    let t11060 = t4881 * t1709 * t4873;
    let t11063 = t4864 * t1709 * t4873;
    let t11065 = -0.59793333333333333333e0_f64 * t10944 + 0.29896666666666666667e0_f64 * t10947 - 0.33218518518518518518e0_f64 * t10951 + 0.11958666666666666667e1_f64 * t10954 - 0.17938e1_f64 * t10960 - 0.29896666666666666667e0_f64 * t10966 - t11030 - t11033 + 0.142419375e1_f64 * t11038 + 0.3071625e0_f64 * t11052 + 0.1898925e1_f64 * t11054 - 0.76790625e-1_f64 * t11057 + 0.46074375e0_f64 * t11060 - 0.28483875e1_f64 * t11063;
    (t11052, t11054, t11057, t11060, t11063, t11065)
}
