//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3625/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3625(t1284: f64, t21333: f64, t68243: f64, t68245: f64, t68247: f64, t68250: f64, t68602: f64, t68604: f64, t68608: f64, t68611: f64, t68613: f64, t68621: f64, t68625: f64, t68628: f64) -> (f64, f64) {
    let t68674 = t21333 * t1284;
    let t68679 = -t68243 - t68245 - t68247 - t68250 - t68602 - t68604 - t68608 - t68611 - t68613 + t68621 + t68625 + t68628;
    (t68674, t68679)
}
