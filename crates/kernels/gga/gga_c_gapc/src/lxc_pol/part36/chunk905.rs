//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 905/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk905<F: Float>(t10024: F, t10027: F, t10032: F, t10037: F, t10041: F, t10044: F, t10048: F, t10051: F, t10054: F, t10059: F, t10064: F, t10070: F, t10073: F, t10080: F) -> F {
    let t11035 = -F::new(0.44080907786205030539e-6) * t10024 + F::new(0.12974218172834570556e-1) * t10027 + F::new(0.10005428175813516294e-7) * t10032 - F::new(0.33764099580923002116e-6) * t10037 - F::new(0.11254699860307667372e-7) * t10041 + F::new(0.82065519814743407918e-9) * t10044 - F::new(0.56273499301538336858e-7) * t10048 + F::new(0.20240885416666666668e-4) * t10051 - F::new(0.49239311888846044752e-7) * t10054 - F::new(0.49239311888846044752e-7) * t10059 + F::new(0.16882049790461501058e-6) * t10064 - F::new(0.50646149371384503174e-6) * t10070 + F::new(0.33764099580923002116e-6) * t10073 - F::new(0.35903664918950240965e-8) * t10080;
    t11035
}
