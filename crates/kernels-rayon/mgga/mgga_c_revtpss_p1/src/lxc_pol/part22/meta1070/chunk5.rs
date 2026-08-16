//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3832/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3832(t22452: f64, t2470: f64, t9680: f64, t1444: f64, t2782: f64, t556: f64, t6895: f64, t9656: f64, t22409: f64, t2435: f64, t13730: f64, t1893: f64) -> (f64, f64, f64, f64) {
    let t73666 = t9680 * t22452 * t2470;
    let t73671 = t2782 * t556 * t9656 * t6895 * t1444;
    let t73673 = t2435 * t22409;
    let t73676 = t2782 * t1893 * t13730;
    (t73666, t73671, t73673, t73676)
}
