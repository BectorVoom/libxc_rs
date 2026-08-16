//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1064/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1064(t34491: f64, t376: f64, t89: f64, t34503: f64, t22873: f64, t28: f64, t6454: f64, t25846: f64, t5507: f64, t3103: f64, t32338: f64, t137215: f64, t137229: f64, t137652: f64, t137654: f64, t137657: f64, t137659: f64, t145667: f64, t145669: f64, t145673: f64, t145676: f64, t145681: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t145684 = t89 * t376 * t34491;
    let t145687 = t89 * t376 * t34503;
    let t145691 = t89 * t28 * t22873 * t6454;
    let t145695 = t89 * t28 * t5507 * t25846;
    let t145699 = t89 * t28 * t32338 * t3103;
    let t145701 = t145667 + t137652 + t137654 - t137657 + 4.0_f64 / 3.0_f64 * t145669 + 2.0_f64 / 9.0_f64 * t145673 - 8.0_f64 / 3.0_f64 * t145676 + t137215 / 3.0_f64 - t137659 - t137229 / 9.0_f64 - 4.0_f64 / 3.0_f64 * t145681 + 2.0_f64 * t145684 - 2.0_f64 / 3.0_f64 * t145687 + 4.0_f64 * t145691 + 4.0_f64 * t145695 - 6.0_f64 * t145699;
    (t145684, t145687, t145691, t145695, t145699, t145701)
}
