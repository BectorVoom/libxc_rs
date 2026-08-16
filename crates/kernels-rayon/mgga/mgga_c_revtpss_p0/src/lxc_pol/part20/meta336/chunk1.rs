//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1260/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1260(t241: f64, t820: f64, t9991: f64, t2482: f64, t4000: f64, t814: f64, t136: f64, t550: f64, t220: f64, t1392: f64, t73: f64, t844: f64) -> (f64, f64, f64, f64, f64) {
    let t13804 = t820 * t9991 * t241;
    let t13845 = t2482 * t4000 * t814;
    let t13846 = t550 * t136;
    let t13847 = t13846 * t220;
    let t13902 = t1392 * t73;
    let t13999 = t820 * t4000 * t844;
    (t13804, t13845, t13847, t13902, t13999)
}
