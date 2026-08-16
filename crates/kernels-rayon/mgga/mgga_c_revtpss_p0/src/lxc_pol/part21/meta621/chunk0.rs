//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2378/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2378(t2237: f64, t2482: f64, t823: f64, t2487: f64, t2646: f64, t2661: f64, t2662: f64, t2663: f64, t10777: f64, t10780: f64, t14686: f64, t10803: f64, t10811: f64) -> (f64, f64, f64, f64, f64) {
    let t40424 = t2482 * t823 * t2237;
    let t40425 = t40424 * t2487;
    let t40429 = t2661 * t2662 * t2663 * t2646;
    let t40438 = t10777 * t14686 * t10780 * t2646;
    let t40440 = t10811 * t10803;
    (t40424, t40425, t40429, t40438, t40440)
}
