//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2502/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2502(t1261: f64, t247: f64, t3363: f64, t44693: f64, t1263: f64, t215: f64, t1122: f64, t12772: f64, t12846: f64, t5331: f64, t12776: f64, t3625: f64) -> (f64, f64, f64, f64, f64) {
    let t44696 = t1261 * t247 * t44693 * t3363;
    let t44701 = t215 * t1263;
    let t44704 = t1261 * t247 * t44701 * t1122;
    let t44711 = t5331 * t12772 * t12846;
    let t44726 = t3625 * t12772 * t12776;
    (t44696, t44701, t44704, t44711, t44726)
}
