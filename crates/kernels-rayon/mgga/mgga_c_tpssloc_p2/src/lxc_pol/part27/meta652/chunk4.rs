//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2278/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2278(t1983: f64, t23857: f64, t7753: f64, t24991: f64, t6876: f64, t12728: f64, t1458: f64, t1459: f64, t16503: f64, t1976: f64, t1980: f64, t23829: f64, t24980: f64, t26103: f64, t4034: f64, t4037: f64, t574: f64, t652: f64, t90034: f64, t90036: f64, t90038: f64, t90040: f64, t90041: f64, t90044: f64, t90051: f64, t90059: f64, t90062: f64, t90064: f64, t90068: f64, t90380: f64, t90411: f64) -> f64 {
    let t90418 = 2.0_f64 * t1983 * t7753 * t23857;
    let t90421 = 6.0_f64 * t6876 * t24991;
    let t90422 = t90034 - t90036 - t90038 + t90040 - 4.0_f64 * t90041 * t1459 - 2.0_f64 * t90044 * t1459 - 4.0_f64 * t26103 * t4037 - t90051 - 4.0_f64 * t4034 * t24980 - 2.0_f64 * t652 * t23829 * t1458 - t90059 + t90062 + t90064 + t90068 + (t90380 + t90411) * t574 - 2.0_f64 * t12728 * t1976 + t90418 + t1980 * t16503 + t90421;
    t90422
}
