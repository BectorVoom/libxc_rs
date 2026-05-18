//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1085/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1085<F: Float>(t1540: F, t1550: F, t2211: F, t2448: F, t3928: F, t39609: F, t43042: F, t43043: F, t45648: F, t45656: F, t45660: F, t45664: F, t45667: F, t45670: F, t45672: F, t45674: F, t45676: F, t45678: F, t45686: F, t45688: F, t47124: F, t6403: F, t699: F) -> F {
    let t48587 = -F::new(0.5107751987195740728e-4) * t45648 - t43042 - F::new(0.4726e1) * t43043 - F::new(0.10215503974391481456e-3) * t45656 + F::new(0.15323255961587222184e-3) * t45660 + F::new(0.10215503974391481456e-3) * t45664 - F::new(0.5107751987195740728e-4) * t45667 + F::new(0.1702583995731913576e-4) * t45670 - F::new(0.5107751987195740728e-4) * t45672 + F::new(0.5107751987195740728e-4) * t45674 + F::new(0.1702583995731913576e-4) * t45676 - F::new(0.1702583995731913576e-4) * t45678 + F::new(0.5107751987195740728e-4) * t45686 - F::new(0.39914139006212695214e-1) * t1540 * t2448 + F::new(0.35922725105591425692e0) * t3928 * t699 * t6403 + F::new(0.23948483403727617128e0) * t1550 * t2211 * t47124 - F::new(0.49658699875514145967e-4) * t45688 - F::new(0.2881692658299671676e-2) * t39609;
    t48587
}
