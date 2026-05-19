//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1024/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1024<F: Float>(t42059: F, t42064: F, t42067: F, t42069: F, t42072: F, t42081: F, t42092: F, t42099: F, t42144: F, t42151: F, t42154: F, t42157: F, t42159: F, t42161: F, t48047: F, t48048: F, t48050: F, t48055: F, t48060: F, t48066: F) -> F {
    let t50887 = -t42059 - t42064 + t42067 - t42069 + t42072 - t42081 + t42092 - t42099 - t48047 - t48048 + F::cast_from(0.14300195980740170668e1_f64) * t48050 - t42144 + t48055 - t42151 + t42154 + F::cast_from(0.55213813373645879536e2_f64) * t48060 + t42157 - t42159 - t42161 + t48066;
    t50887
}
