//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1195/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1195(t2131: f64, t309: f64, t8004: f64, t9976: f64, t2138: f64, t322: f64, t26757: f64, t32124: f64, t33008: f64, t33015: f64, t33019: f64, t33028: f64, t38033: f64, t38036: f64, t38051: f64, t38055: f64, t38065: f64, t38635: f64, t8306: f64, t8400: f64, t9003: f64, t9414: f64, t9427: f64) -> f64 {
    let t41075 = t2131 * t8004 * t9976 * t309;
    let t41079 = t2138 * t8004 * t9976 * t322;
    let t41086 = 0.65854491829355115987e0_f64 * t33008 - t38033 + t38036 + 0.17347256376410398924e1_f64 * t9003 * t9414 + 0.26020884564615598386e1_f64 * t32124 * t8306 * t38635 + t33015 + t33019 - 0.52041769129231196772e1_f64 * t41075 + 0.52041769129231196772e1_f64 * t41079 - 0.8673628188205199462e0_f64 * t8400 * t9427 * t26757 + t38051 + t38055 + 0.17347256376410398924e1_f64 * t38065 + 0.34694512752820797848e1_f64 * t33028;
    t41086
}
