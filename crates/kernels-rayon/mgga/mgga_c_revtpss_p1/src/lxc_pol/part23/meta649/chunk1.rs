//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2375/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2375(t2237: f64, t2482: f64, t823: f64, t2487: f64, t10111: f64, t849: f64, t9720: f64, t685: f64, t775: f64, t855: f64, t242: f64, t240: f64, t72: f64) -> (f64, f64, f64, f64, f64) {
    let t40424 = t2482 * t823 * t2237;
    let t40425 = t40424 * t2487;
    let t40452 = t10111 * t849 * t9720;
    let t40455 = t40452 * t855 * t685 * t775;
    let t40459 = t242 * t242;
    let t40460 = 1.0_f64 / t40459;
    let t40462 = t240 * t40460 * t72;
    (t40424, t40425, t40452, t40455, t40462)
}
