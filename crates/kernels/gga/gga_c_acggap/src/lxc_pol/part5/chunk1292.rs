//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1292/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1292<F: Float>(t1111: F, t1181: F, t22705: F, t3391: F, t17550: F, t6324: F, t1180: F, t13286: F, t13287: F, t13364: F, t14292: F, t14297: F, t14301: F, t14309: F, t14313: F, t1754: F, t17656: F, t17972: F, t18566: F, t20987: F, t525: F, t6269: F, t6394: F, t8401: F) -> F {
    let t23991 = t3391 * t1181 * t22705 * t1111;
    let t23994 = t17550 * t6324;
    let t23996 = t14292 - F::cast_from(0.12004725073059526352e-1_f64) * t14297 + t14301 - F::cast_from(0.22675591804667994221e-1_f64) * t14309 + F::cast_from(0.85748036236139473944e-3_f64) * t14313 + F::cast_from(0.34299214494455789578e-2_f64) * t17656 * t13287 * t8401 * t6394 - F::cast_from(0.13719685797782315831e-1_f64) * t13286 * t13287 * t525 * t20987 + F::cast_from(0.68598428988911579156e-2_f64) * t13286 * t13364 * t8401 * t6269 - F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t17972 * t1754 + F::cast_from(0.17149607247227894789e-2_f64) * t23991 + F::cast_from(0.85748036236139473944e-3_f64) * t18566 + F::cast_from(0.16006300097412701803e0_f64) * t23994;
    t23996
}
