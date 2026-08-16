//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3153/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3153(t12772: f64, t17639: f64, t3625: f64, t17645: f64, t1284: f64, t17288: f64, t3624: f64, t12917: f64, t17401: f64, t17396: f64, t1260: f64, t17289: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57026 = t3625 * t12772 * t17639;
    let t57029 = t3625 * t12772 * t17645;
    let t57040 = t17288 * t1284 * t3624;
    let t57045 = t17401 * t12917;
    let t57049 = t17396 * t12917;
    let t57053 = t17289 * t1260;
    (t57026, t57029, t57040, t57045, t57049, t57053)
}
