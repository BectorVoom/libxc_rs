//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 952/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk952<F: Float>(t10024: F, t10027: F, t10032: F, t10037: F, t10041: F, t10044: F, t10048: F, t10051: F, t10054: F, t10059: F, t10064: F, t10070: F, t10073: F, t10080: F) -> F {
    let t11035 = -F::cast_from(0.44080907786205030539e-6_f64) * t10024 + F::cast_from(0.12974218172834570556e-1_f64) * t10027 + F::cast_from(0.10005428175813516294e-7_f64) * t10032 - F::cast_from(0.33764099580923002116e-6_f64) * t10037 - F::cast_from(0.11254699860307667372e-7_f64) * t10041 + F::cast_from(0.82065519814743407918e-9_f64) * t10044 - F::cast_from(0.56273499301538336858e-7_f64) * t10048 + F::cast_from(0.20240885416666666668e-4_f64) * t10051 - F::cast_from(0.49239311888846044752e-7_f64) * t10054 - F::cast_from(0.49239311888846044752e-7_f64) * t10059 + F::cast_from(0.16882049790461501058e-6_f64) * t10064 - F::cast_from(0.50646149371384503174e-6_f64) * t10070 + F::cast_from(0.33764099580923002116e-6_f64) * t10073 - F::cast_from(0.35903664918950240965e-8_f64) * t10080;
    t11035
}
