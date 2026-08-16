//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1551/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1551(t18124: f64, t18164: f64, t1055: f64, t1052: f64, t1066: f64, t14529: f64, t14545: f64, t14552: f64, t14555: f64, t1635: f64, t18053: f64, t18057: f64, t18059: f64, t18062: f64, t18065: f64, t18071: f64, t18074: f64, t388: f64, t4660: f64, t4665: f64) -> f64 {
    let t18165 = t18124 + t18164;
    let t18166 = t1055 * t18165;
    let t18168 = 2.0_f64 * t1052 * t18062 - 6.0_f64 * t1052 * t18071 - t1052 * t18166 - t1066 * t18074 - 2.0_f64 * t14529 * t1635 - 2.0_f64 * t14545 * t1635 - 2.0_f64 * t14552 * t1635 - 2.0_f64 * t14555 * t1635 + t18053 * t388 + t18057 * t388 + t18059 * t388 + 2.0_f64 * t18065 * t388 + 4.0_f64 * t4660 * t4665;
    t18168
}
