//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 953/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk953(t10024: f64, t10027: f64, t10032: f64, t10037: f64, t10041: f64, t10044: f64, t10048: f64, t10051: f64, t10054: f64, t10059: f64, t10064: f64, t10070: f64, t10073: f64, t10080: f64) -> f64 {
    let t11035 = -0.44080907786205030539e-6_f64 * t10024 + 0.12974218172834570556e-1_f64 * t10027 + 0.10005428175813516294e-7_f64 * t10032 - 0.33764099580923002116e-6_f64 * t10037 - 0.11254699860307667372e-7_f64 * t10041 + 0.82065519814743407918e-9_f64 * t10044 - 0.56273499301538336858e-7_f64 * t10048 + 0.20240885416666666668e-4_f64 * t10051 - 0.49239311888846044752e-7_f64 * t10054 - 0.49239311888846044752e-7_f64 * t10059 + 0.16882049790461501058e-6_f64 * t10064 - 0.50646149371384503174e-6_f64 * t10070 + 0.33764099580923002116e-6_f64 * t10073 - 0.35903664918950240965e-8_f64 * t10080;
    t11035
}
