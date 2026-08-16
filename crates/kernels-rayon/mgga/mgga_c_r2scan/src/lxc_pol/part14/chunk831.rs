//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 831/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk831(t1620: f64, t2682: f64, t129: f64, t1598: f64, t524: f64, t2593: f64, t1610: f64, t2207: f64, t2691: f64, t2530: f64, t537: f64, t2124: f64, t2551: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7490 = t1620 * t2682;
    let t7494 = t524 * t1598 * t129;
    let t7496 = 0.25610080155860322884e0_f64 * t7494 * t2593;
    let t7500 = 0.34930954652346593434e-1_f64 * t2207 * t1610 * t2691;
    let t7503 = t537 * t2530;
    let t7505 = t2124 * t7503 * t2551;
    (t7490, t7494, t7496, t7500, t7503, t7505)
}
