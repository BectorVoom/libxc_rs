//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1028/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1028(t1444: f64, t32211: f64, t5673: f64, t32206: f64, t4075: f64, t8705: f64, t1419: f64, t8477: f64, t1385: f64, t9656: f64) -> (f64, f64, f64, f64, f64) {
    let t32213 = t5673 * t32211 * t1444;
    let t32214 = t32206 * t32213;
    let t32237 = t8705 * t4075;
    let t32247 = t8477 * t1419;
    let t32250 = t9656 * t1385;
    (t32213, t32214, t32237, t32247, t32250)
}
