//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1076/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1076(t174: f64, t18431: f64, t447: f64, t637: f64, t446: f64, t1658: f64, t5398: f64, t233: f64, t1885: f64, t6260: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t175 = t174 <= zeta_threshold;
    let t18432 = piecewise3(t175, 0.0_f64, t18431);
    let t18433 = t447 * t18432;
    let t18434 = t18433 * t637;
    let t18435 = t446 * t18434;
    let t18437 = t1658 * t5398;
    let t18438 = t233 * t18437;
    let t18440 = t1885 * t6260;
    let t18441 = t446 * t18440;
    let t18443 = -t18431;
    (t18435, t18438, t18441, t18443)
}
