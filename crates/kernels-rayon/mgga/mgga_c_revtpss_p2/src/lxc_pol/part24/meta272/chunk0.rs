//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1045/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1045(t221: f64, t2485: f64, t6022: f64, t10850: f64, t14718: f64, t6035: f64, t2662: f64, t2661: f64, t125: f64, t6016: f64, t2741: f64, t5980: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18432 = t2485 * t221 * t6022;
    let t18433 = t10850 * t18432;
    let t18440 = t14718 * t6035;
    let t18441 = t2662 * t18440;
    let t18442 = t2661 * t18441;
    let t18444 = t125 * t6016;
    let t18459 = t2741 * t5980;
    (t18432, t18433, t18441, t18442, t18444, t18459)
}
