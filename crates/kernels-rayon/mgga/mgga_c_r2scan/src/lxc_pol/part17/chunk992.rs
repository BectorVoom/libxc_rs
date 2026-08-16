//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 992/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk992(t11845: f64, t10871: f64, t10887: f64, t10893: f64, t10896: f64, t10898: f64, t10906: f64, t11444: f64, t11835: f64, t11838: f64, t11840: f64, t12192: f64) -> f64 {
    let t12193 = 0.12805040077930161442e0_f64 * t11845;
    let t12194 = -t10871 - 0.86682217400542685632e-1_f64 * t11835 - 0.86682217400542685632e-1_f64 * t11838 - 0.86682217400542685632e-1_f64 * t11840 + t12192 + t12193 + t10887 + t10893 + t10896 - t10898 - t11444 + t10906;
    t12194
}
