//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 377/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk377(t1737: f64, t1746: f64, t2430: f64, t1706: f64, t1735: f64, t2404: f64, t2418: f64, t2422: f64, t45: f64, t621: f64, t634: f64, t67: f64) -> (f64, f64) {
    let t2432 = t1737 * t2430 * t1746;
    let t2435 = -0.62182e-1_f64 * t2404 * t621 + 1.0_f64 * t1706 * t2418 + 0.19751789702565206229e-1_f64 * t45 * t2422 * t634 - 0.58482233974552040708e0_f64 * t1735 * t2432;
    let t2436 = t67 * t2435;
    (t2432, t2436)
}
