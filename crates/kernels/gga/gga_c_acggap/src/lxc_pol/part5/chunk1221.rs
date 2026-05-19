//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1221/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1221<F: Float>(t1077: F, t1083: F, t1089: F, t13286: F, t13287: F, t1459: F, t15386: F, t17171: F, t17177: F, t17198: F, t1772: F, t1795: F, t1849: F, t22325: F, t22327: F, t22329: F, t22333: F, t22337: F, t3169: F, t3176: F, t368: F, t398: F, t418: F, t4838: F, t513: F, t839: F) -> F {
    let t22339 = -F::cast_from(0.13719685797782315831e-1_f64) * t13286 * t13287 * t1849 * t3169 + F::cast_from(0.20579528696673473746e-1_f64) * t13286 * t15386 * t1849 * t3176 - F::cast_from(0.85748036236139473944e-3_f64) * t418 * t398 * t1083 * t1795 * t1077 + F::cast_from(0.25724410870841842184e-2_f64) * t418 * t398 * t1459 * t513 * t4838 + F::cast_from(0.68598428988911579156e-2_f64) * t17171 - F::cast_from(0.34299214494455789578e-2_f64) * t17177 - F::cast_from(0.34299214494455789578e-2_f64) * t17198 - F::cast_from(0.17149607247227894789e-2_f64) * t418 * t1089 * t368 * t1772 * t839 - F::cast_from(0.21437009059034868486e-3_f64) * t22325 + F::cast_from(0.34299214494455789578e-2_f64) * t22327 - F::cast_from(0.17149607247227894789e-2_f64) * t22329 - F::cast_from(0.17149607247227894789e-2_f64) * t22333 - F::cast_from(0.34299214494455789578e-2_f64) * t22337;
    t22339
}
