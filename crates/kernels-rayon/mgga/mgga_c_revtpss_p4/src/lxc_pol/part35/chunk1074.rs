//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1074/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1074(t2097: f64, t3999: f64, t531: f64, t8107: f64, t116: f64, t7983: f64, t1450: f64, t6816: f64, t6836: f64, t196: f64, t197: f64, t6773: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28911 = t3999 * t2097;
    let t28938 = t531 * t8107;
    let t28986 = t116 * t7983;
    let t29494 = t1450 * t6816;
    let t29498 = t1450 * t6836;
    let t29506 = t6773 * t196 * t197;
    (t28911, t28938, t28986, t29494, t29498, t29506)
}
