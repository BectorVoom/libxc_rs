//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2788/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2788(t10111: f64, t823: f64, t9720: f64, t685: f64, t827: f64, t837: f64, t10837: f64, t9775: f64, t2237: f64, t2482: f64, t2487: f64, t849: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40406 = t10111 * t823 * t9720;
    let t40409 = t40406 * t827 * t685 * t837;
    let t40411 = t9775 * t10837;
    let t40424 = t2482 * t823 * t2237;
    let t40425 = t40424 * t2487;
    let t40452 = t10111 * t849 * t9720;
    (t40406, t40409, t40411, t40424, t40425, t40452)
}
