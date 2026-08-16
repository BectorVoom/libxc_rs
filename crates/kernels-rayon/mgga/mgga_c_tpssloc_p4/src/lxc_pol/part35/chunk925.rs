//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 925/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk925(t4997: f64, t5002: f64, t11784: f64, t248: f64, t5971: f64, t1227: f64, t5019: f64, t4993: f64, t5005: f64, t5024: f64, t1017: f64, t6163: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18972 = t5002 * t4997;
    let t18975 = t248 * t11784 * t5971;
    let t18976 = t1227 * t18975;
    let t18978 = t5019 * t4997;
    let t18980 = t5005 * t4993;
    let t18987 = t5024 * t4993;
    let t19024 = t6163 * t1017;
    (t18972, t18975, t18976, t18978, t18980, t18987, t19024)
}
