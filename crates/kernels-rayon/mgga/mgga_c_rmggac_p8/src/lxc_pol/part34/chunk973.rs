//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 973/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk973(t74693: f64, t74695: f64, t74698: f64, t74701: f64, t74705: f64, t71033: f64, t74662: f64, t74667: f64, t74670: f64, t74677: f64, t77143: f64, t77148: f64, t77150: f64, t77154: f64, t77155: f64, t77156: f64, t77157: f64) -> f64 {
    let t77158 = 0.1276937996798935182e-4_f64 * t74693;
    let t77159 = 0.1276937996798935182e-4_f64 * t74695;
    let t77160 = 0.3192344991997337955e-4_f64 * t74698;
    let t77161 = 0.2627895913935205078e-5_f64 * t74701;
    let t77162 = 0.5255791827870410156e-5_f64 * t74705;
    let t77163 = -t71033 + 0.35038612185802734376e-6_f64 * t74662 + 0.8759653046450683594e-6_f64 * t74667 - t74670 - t77143 + 0.76860658247009135557e-5_f64 * t74677 - t77148 + t77150 + t77154 - t77155 + t77156 - t77157 - t77158 + t77159 + t77160 + t77161 - t77162;
    t77163
}
