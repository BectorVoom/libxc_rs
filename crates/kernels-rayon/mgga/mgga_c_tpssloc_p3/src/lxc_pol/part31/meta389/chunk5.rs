//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1393/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1393(t5914: f64, t990: f64, t17875: f64, t381: f64, t1049: f64, t5848: f64, t1065: f64, t5943: f64, t3174: f64, t1625: f64, t4552: f64, t5919: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18053 = t990 * t5914;
    let t18057 = t17875 * t381;
    let t18059 = t5848 * t1049;
    let t18061 = t5943 * t1065;
    let t18062 = t3174 * t18061;
    let t18065 = t4552 * t1625;
    let t18070 = t5919 * t1065;
    (t18053, t18057, t18059, t18062, t18065, t18070)
}
