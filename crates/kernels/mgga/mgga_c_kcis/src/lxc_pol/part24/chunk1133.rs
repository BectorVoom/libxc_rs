//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1133/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1133<F: Float>(t1281: F, t29186: F, t100026: F, t100029: F, t100031: F, t100033: F, t1291: F, t15692: F, t2205: F, t28073: F, t28253: F, t28295: F, t29188: F, t29214: F, t34650: F, t35615: F, t3664: F, t3669: F, t47711: F, t5360: F, t6879: F, t70451: F, t7823: F, t8108: F, t99859: F, t99861: F, t99864: F) -> (F,) {
    let t100920 = t29186 * t1281;
    let t100927 = 24.0 * t1291 * t29188 * t35615 + 2.0 * t1291 * t29214 * t3669 + 2.0 * t3669 * t6879 * t7823 - t100920 * t1291 + 4.0 * t15692 * t28073 + 4.0 * t15692 * t28253 - t2205 * t70451 - 2.0 * t28295 * t5360 - 6.0 * t29188 * t34650 - t29214 * t3664 + 4.0 * t47711 * t8108 + t100026 + t100029 + t100031 - t100033 - t99859 - t99861 + t99864;
    (t100927,)
}
