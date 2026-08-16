//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 509/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk509(t235: f64, t3999: f64, t543: f64, t531: f64, t549: f64, t240: f64, t72: f64, t1386: f64, t2482: f64, t27: f64, t136: f64, t1389: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4000 = t3999 * t235;
    let t4003 = t543 * t543;
    let t4010 = 1.0_f64 / t549 / t531;
    let t4011 = t240 * t4010;
    let t4012 = t4011 * t72;
    let t4018 = t2482 * t1386 * t27;
    let t4019 = t1389 * t136;
    (t4000, t4003, t4010, t4012, t4018, t4019)
}
