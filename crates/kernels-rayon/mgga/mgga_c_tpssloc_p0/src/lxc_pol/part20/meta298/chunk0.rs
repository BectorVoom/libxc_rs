//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1515/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1515(t3120: f64, t3188: f64, t1059: f64, t10471: f64, t10474: f64, t10470: f64, t10482: f64, t6739: f64, t11047: f64, t3127: f64, t3131: f64, t1049: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11054 = t3188 * t3120;
    let t11055 = t1059 * t11054;
    let t11058 = t10471 * t10474;
    let t11059 = t10470 * t11058;
    let t11060 = t6739 * t10482;
    let t11061 = t11047 * t11060;
    let t11064 = t10471 * t3127;
    let t11065 = t10470 * t11064;
    let t11066 = t6739 * t3131;
    let t11067 = t11047 * t11066;
    let t11077 = t1049 * t3120;
    (t11054, t11055, t11058, t11059, t11060, t11061, t11064, t11065, t11066, t11067, t11077)
}
