//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2565/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2565(t14536: f64, t225: f64, t10164: f64, t1634: f64, t14532: f64, t14562: f64, t14527: f64, t14534: f64, t11190: f64, t1670: f64, t3242: f64, t457: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t50625 = t14536 * t225;
    let t50628 = t10164 * t1634;
    let t50632 = t14532 * t225;
    let t50653 = t14562 * t225;
    let t50690 = t14527 * t225;
    let t50703 = t14534 * t225;
    let t50819 = t11190 * t1670;
    let t50822 = t457 * t3242;
    (t50625, t50628, t50632, t50653, t50690, t50703, t50819, t50822)
}
