//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1007/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1007<F: Float>(t1450: F, t26987: F, t1415: F, t1411: F, t3924: F, t8059: F, t1327: F, t13440: F, t8063: F, t1220: F, t13437: F, t14224: F, t14226: F, t14242: F, t20002: F, t26948: F, t26952: F, t26956: F, t26960: F, t26962: F, t26964: F, t26967: F, t26970: F, t26976: F, t26980: F, t3930: F, t8064: F) -> (F, F, F, F, F) {
    let t26988 = t1450 * t26987;
    let t26989 = t1415 * t26988;
    let t26990 = t1411 * t26989;
    let t26992 = t8059 * t3924;
    let t26993 = t26992 * t1327;
    let t26998 = t8063 * t13440;
    let t26999 = t26998 * t1327;
    let t27002 = -0.73697530864197530861e-2 * t26948 + 0.73697530864197530861e-2 * t26952 + 0.11054629629629629629e-1 * t26956 + 0.11054629629629629629e-2 * t26960 - 0.33163888888888888888e-2 * t26962 + 0.22109259259259259259e-2 * t26964 - 0.49745833333333333332e-2 * t26967 - 0.88437037037037037035e-2 * t26970 - 0.22109259259259259259e-2 * t20002 - 0.16581944444444444444e-2 * t26976 + 0.55273148148148148147e-3 * t14224 - 0.36848765432098765431e-3 * t14226 + 0.148996e0 * t3930 * t26980 + 0.74498e-1 * t14242 * t8064 + 0.386e0 * t1220 * t26980 - 0.16581944444444444444e-2 * t26990 + 0.193e0 * t1220 * t26993 + 0.74498e-1 * t3930 * t26993 - 0.43134342e-1 * t13437 * t26999;
    (t26990, t26992, t26998, t26999, t27002)
}
