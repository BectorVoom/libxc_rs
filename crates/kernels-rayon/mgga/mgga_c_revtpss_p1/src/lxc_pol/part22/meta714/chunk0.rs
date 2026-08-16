//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2745/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2745(t2452: f64, t588: f64, t258: f64, t2454: f64, t2455: f64, t39494: f64, t10985: f64, t11018: f64, t2439: f64, t2760: f64, t780: f64, t785: f64) -> (f64, f64, f64, f64, f64) {
    let t39552 = t588 * t2452;
    let t39554 = 0.88356352675825229576e-3_f64 * t39552 * t258;
    let t39557 = 0.20561456923286030469e-1_f64 * t2454 * t2455 * t39494;
    let t39558 = t11018 * t10985;
    let t39562 = t2439 * t785 * t2760 * t780;
    (t39552, t39554, t39557, t39558, t39562)
}
