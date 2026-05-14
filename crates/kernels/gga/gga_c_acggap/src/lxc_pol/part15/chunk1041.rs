//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1041/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1041<F: Float>(t2131: F, t309: F, t8004: F, t9976: F, t2138: F, t322: F, t26757: F, t32124: F, t33008: F, t33015: F, t33019: F, t33028: F, t38033: F, t38036: F, t38051: F, t38055: F, t38065: F, t38635: F, t8306: F, t8400: F, t9003: F, t9414: F, t9427: F) -> (F,) {
    let t41075 = t2131 * t8004 * t9976 * t309;
    let t41079 = t2138 * t8004 * t9976 * t322;
    let t41086 = 0.65854491829355115987e0 * t33008 - t38033 + t38036 + 0.17347256376410398924e1 * t9003 * t9414 + 0.26020884564615598386e1 * t32124 * t8306 * t38635 + t33015 + t33019 - 0.52041769129231196772e1 * t41075 + 0.52041769129231196772e1 * t41079 - 0.8673628188205199462e0 * t8400 * t9427 * t26757 + t38051 + t38055 + 0.17347256376410398924e1 * t38065 + 0.34694512752820797848e1 * t33028;
    (t41086,)
}
