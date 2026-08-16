//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 796/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk796(t3255: f64, t4603: f64, t4608: f64, t1071: f64, t1114: f64, t4634: f64, t4597: f64, t1035: f64, t3293: f64, t1727: f64, t934: f64, t313: f64, t4600: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14125 = 0.98556445e-3_f64 * t3255 * t4603;
    let t14127 = 0.19711289e-2_f64 * t3255 * t4608;
    let t14128 = t1114 * t1071;
    let t14137 = t3255 * t4634;
    let t14168 = 0.13140859333333333333e-2_f64 * t3255 * t4597;
    let t14170 = t3293 * t1035;
    let t14171 = t1727 * t934;
    let t14196 = t4600 * t313;
    (t14125, t14127, t14128, t14137, t14168, t14170, t14171, t14196)
}
