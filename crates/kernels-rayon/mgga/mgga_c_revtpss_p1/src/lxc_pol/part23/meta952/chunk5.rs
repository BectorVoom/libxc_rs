//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3160/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3160(t21272: f64, t5378: f64, t44799: f64, t82578: f64, t1794: f64, t5825: f64, t1250: f64, t1469: f64, t4186: f64, t12772: f64, t24793: f64, t3625: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83018 = t21272 * t5378;
    let t83024 = t82578 * t44799;
    let t83033 = t5825 * t1794;
    let t83034 = t83033 * t44799;
    let t83040 = t1469 * t1794 * t1250 * t4186;
    let t83047 = t3625 * t12772 * t24793;
    (t83018, t83024, t83033, t83034, t83040, t83047)
}
