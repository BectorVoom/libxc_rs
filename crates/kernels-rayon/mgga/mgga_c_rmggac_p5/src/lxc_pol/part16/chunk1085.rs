//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1085/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1085(t1540: f64, t1550: f64, t2211: f64, t2448: f64, t3928: f64, t39609: f64, t43042: f64, t43043: f64, t45648: f64, t45656: f64, t45660: f64, t45664: f64, t45667: f64, t45670: f64, t45672: f64, t45674: f64, t45676: f64, t45678: f64, t45686: f64, t45688: f64, t47124: f64, t6403: f64, t699: f64) -> f64 {
    let t48587 = -0.5107751987195740728e-4_f64 * t45648 - t43042 - 0.4726e1_f64 * t43043 - 0.10215503974391481456e-3_f64 * t45656 + 0.15323255961587222184e-3_f64 * t45660 + 0.10215503974391481456e-3_f64 * t45664 - 0.5107751987195740728e-4_f64 * t45667 + 0.1702583995731913576e-4_f64 * t45670 - 0.5107751987195740728e-4_f64 * t45672 + 0.5107751987195740728e-4_f64 * t45674 + 0.1702583995731913576e-4_f64 * t45676 - 0.1702583995731913576e-4_f64 * t45678 + 0.5107751987195740728e-4_f64 * t45686 - 0.39914139006212695214e-1_f64 * t1540 * t2448 + 0.35922725105591425692e0_f64 * t3928 * t699 * t6403 + 0.23948483403727617128e0_f64 * t1550 * t2211 * t47124 - 0.49658699875514145967e-4_f64 * t45688 - 0.2881692658299671676e-2_f64 * t39609;
    t48587
}
