//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 902/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk902(t3: f64, t8692: f64, t1873: f64, t7423: f64, t577: f64, t8503: f64, t8506: f64, t8508: f64, t192: f64, t533: f64, t1390: f64, t2018: f64) -> (f64, f64, f64, f64) {
    let t8693 = t3 * t8692;
    let t8699 = t7423 * t1873;
    let t8702 = 0.45e1_f64 * t8692 * t577 + 0.135e2_f64 * t8699 + 0.135e2_f64 * t8503 + t8506 + t8508;
    let t8944 = t192 * t533;
    let t8945 = t2018 * t1390;
    (t8693, t8702, t8944, t8945)
}
