//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 814/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk814<F: Float>(t10037: F, t967: F, t12323: F, t159: F, t285: F, t11290: F, t281: F, t6028: F, t6032: F, t6036: F, t6043: F, t6049: F, t6053: F, t6058: F, t6064: F, t8503: F) -> (F, F, F) {
    let t13057 = t10037 * t967;
    let t13062 = t12323 * t159 * t285;
    let t13067 = t6028 - t6032 - t6036 - F::cast_from(0.11974234010254609094e-1_f64) * t281 * t13062 + F::cast_from(0.11974234010254609094e0_f64) * t8503 - t6043 + t6049 - t6053 - t6058 + t6064 + F::cast_from(0.59871170051273045469e-1_f64) * t11290;
    (t13057, t13062, t13067)
}
