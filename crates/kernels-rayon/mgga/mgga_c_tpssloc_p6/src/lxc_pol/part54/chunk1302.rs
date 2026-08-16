//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1302/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1302(t111: f64, t32348: f64, t112: f64, t32392: f64, t8843: f64, t25: f64, t25353: f64, t606: f64, t7540: f64, t1408: f64, t6665: f64, t1530: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t117533 = t32348 * t111;
    let t117672 = t32392 * t112;
    let t117687 = t8843 * t111;
    let t118387 = t25 * t25353;
    let t118393 = t606 * t7540;
    let t118410 = t1408 * t6665;
    let t118413 = t1530 * t6665;
    (t117533, t117672, t117687, t118387, t118393, t118410, t118413)
}
