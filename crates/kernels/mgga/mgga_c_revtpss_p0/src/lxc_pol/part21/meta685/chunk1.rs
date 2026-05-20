//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2502/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2502<F: Float>(t1261: F, t247: F, t3363: F, t44693: F, t1263: F, t215: F, t1122: F, t12772: F, t12846: F, t5331: F, t12776: F, t3625: F) -> (F, F, F, F, F) {
    let t44696 = t1261 * t247 * t44693 * t3363;
    let t44701 = t215 * t1263;
    let t44704 = t1261 * t247 * t44701 * t1122;
    let t44711 = t5331 * t12772 * t12846;
    let t44726 = t3625 * t12772 * t12776;
    (t44696, t44701, t44704, t44711, t44726)
}
