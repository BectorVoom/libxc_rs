//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1189/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1189(t1891: f64, t7614: f64, t1998: f64, t6125: f64, t30811: f64, t6090: f64, t31682: f64, t31684: f64, t35952: f64, t35962: f64, t35964: f64, t35968: f64, t35969: f64, t35973: f64, t35976: f64, t35978: f64, t35980: f64, t35982: f64, t37800: f64, t37803: f64, t37806: f64) -> f64 {
    let t40385 = t7614 * t1891;
    let t40387 = t1998 * t6125;
    let t40390 = t30811 * t6090;
    let t40394 = 0.27953859812981468504e-2_f64 * t31682 + 0.80031500487063509015e-2_f64 * t40385 - 0.85748036236139473944e-3_f64 * t40387 - 0.31448092289604152068e-3_f64 * t31684 - t37800 - t35952 + t37803 + t37806 + t35962 + t35964 - t35968 - 0.68598428988911579156e-2_f64 * t40390 + 0.80031500487063509015e-2_f64 * t35969 - 0.80031500487063509015e-2_f64 * t35973 - t35976 + t35978 - t35980 + t35982;
    t40394
}
