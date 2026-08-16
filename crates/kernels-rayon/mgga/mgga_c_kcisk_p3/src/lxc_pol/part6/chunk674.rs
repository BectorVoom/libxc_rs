//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 674/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk674(t4789: f64, t599: f64, t4740: f64, t583: f64, t573: f64, t10568: f64, t10641: f64, t1643: f64, t4743: f64, t586: f64, t657: f64, t963: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10699 = 1.0_f64 / t4789 / t599;
    let t10714 = 1.0_f64 / t4740 / t583;
    let t10715 = t573 * t10714;
    let t10738 = 0.93011851851851851854e0_f64 * t10568;
    let t10739 = 0.36514074074074074075e0_f64 * t10641;
    let t10754 = 1.0_f64 / t4740 / t1643;
    let t10755 = t573 * t10754;
    let t10757 = 1.0_f64 / t4743 / t586;
    let t10761 = 0.28842592592592592592e-1_f64 * t10568;
    let t10791 = t963 * t657;
    (t10699, t10715, t10738, t10739, t10755, t10757, t10761, t10791)
}
