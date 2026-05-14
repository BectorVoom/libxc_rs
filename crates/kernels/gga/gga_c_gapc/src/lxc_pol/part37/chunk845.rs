//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 845/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk845<F: Float>(t10024: F, t10027: F, t10032: F, t10037: F, t10041: F, t10044: F, t10048: F, t10051: F, t10054: F, t10059: F, t10064: F, t10070: F, t10073: F, t10080: F, t10813: F, t10827: F, t10842: F, t10856: F, t10872: F, t10886: F, t10901: F, t10915: F, t10932: F, t10946: F, t10961: F, t10975: F, t10991: F, t11005: F, t11020: F) -> (F,) {
    let t11035 = -0.44080907786205030539e-6 * t10024 + 0.12974218172834570556e-1 * t10027 + 0.10005428175813516294e-7 * t10032 - 0.33764099580923002116e-6 * t10037 - 0.11254699860307667372e-7 * t10041 + 0.82065519814743407918e-9 * t10044 - 0.56273499301538336858e-7 * t10048 + 0.20240885416666666668e-4 * t10051 - 0.49239311888846044752e-7 * t10054 - 0.49239311888846044752e-7 * t10059 + 0.16882049790461501058e-6 * t10064 - 0.50646149371384503174e-6 * t10070 + 0.33764099580923002116e-6 * t10073 - 0.35903664918950240965e-8 * t10080;
    let t11039 = t10813 + t10827 + t10842 + t10856 + t10872 + t10886 + t10901 + t10915 + t10932 + t10946 + t10961 + t10975 + t10991 + t11005 + t11020 + t11035;
    (t11039,)
}
