//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1099/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1099<F: Float>(t1846: F, t3476: F, t1008: F, t6220: F, t6228: F, t384: F, t398: F, t4623: F, t535: F, t1111: F, t1165: F, t20400: F, t3361: F, t1077: F, t1083: F, t1089: F, t13286: F, t13287: F, t1459: F, t15386: F, t17171: F, t17177: F, t17198: F, t1772: F, t1795: F, t1849: F, t3169: F, t3176: F, t368: F, t418: F, t4838: F, t513: F, t839: F) -> (F,) {
    let t22325 = t3476 * t1846;
    let t22327 = t1008 * t6220;
    let t22329 = t1008 * t6228;
    let t22333 = t384 * t398 * t535 * t4623;
    let t22337 = t3361 * t1165 * t20400 * t1111;
    let t22339 = -0.13719685797782315831e-1 * t13286 * t13287 * t1849 * t3169 + 0.20579528696673473746e-1 * t13286 * t15386 * t1849 * t3176 - 0.85748036236139473944e-3 * t418 * t398 * t1083 * t1795 * t1077 + 0.25724410870841842184e-2 * t418 * t398 * t1459 * t513 * t4838 + 0.68598428988911579156e-2 * t17171 - 0.34299214494455789578e-2 * t17177 - 0.34299214494455789578e-2 * t17198 - 0.17149607247227894789e-2 * t418 * t1089 * t368 * t1772 * t839 - 0.21437009059034868486e-3 * t22325 + 0.34299214494455789578e-2 * t22327 - 0.17149607247227894789e-2 * t22329 - 0.17149607247227894789e-2 * t22333 - 0.34299214494455789578e-2 * t22337;
    (t22339,)
}
