//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1023/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1023(t248: f64, t3516: f64, t3570: f64, t3515: f64, t11154: f64, t3585: f64, t3493: f64, t486: f64, t4978: f64, t4582: f64, t3576: f64, t3604: f64) -> (f64, f64, f64, f64, f64) {
    let t11651 = t248 * t3570 * t3516;
    let t11652 = t3515 * t11651;
    let t11655 = t248 * t3585 * t11154;
    let t11660 = t486 * t3493;
    let t11661 = t11660 * t4978;
    let t11662 = t4582 * t11661;
    let t11665 = t3604 * t3576;
    (t11652, t11655, t11660, t11662, t11665)
}
