//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1191/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1191<F: Float>(t1165: F, t5697: F, t7575: F, t8600: F, t31693: F, t31700: F, t31702: F, t31704: F, t31708: F, t31721: F, t36042: F, t36066: F, t36082: F, t36086: F, t36088: F, t36090: F, t36097: F, t36119: F, t36123: F, t37858: F, t37869: F) -> F {
    let t40418 = t7575 * t1165 * t8600 * t5697;
    let t40422 = -t36042 - t36066 + F::cast_from(0.7145669686344956162e-3_f64) * t31693 - F::cast_from(0.14291339372689912324e-3_f64) * t31700 + F::cast_from(0.15724046144802076034e-3_f64) * t31702 + F::cast_from(0.20965394859736101378e-3_f64) * t31704 + F::cast_from(0.28582678745379824648e-3_f64) * t31708 + t36082 - t31721 + t37858 + F::cast_from(0.18868855373762491241e-1_f64) * t40418 + t36086 + t36088 - t36090 + t36097 - t37869 + F::cast_from(0.41930789719472202756e-3_f64) * t36119 - F::cast_from(0.62896184579208304134e-3_f64) * t36123;
    t40422
}
