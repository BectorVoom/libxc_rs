//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 889/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk889(t3: f64, t8843: f64, t2039: f64, t577: f64, t7423: f64, t8508: f64, t8654: f64, t8659: f64, t192: f64, t533: f64) -> (f64, f64, f64) {
    let t8844 = t3 * t8843;
    let t8852 = 0.45e1_f64 * t8843 * t577 + 0.135e2_f64 * t7423 * t2039 + t8654 + t8659 + t8508;
    let t8944 = t192 * t533;
    (t8844, t8852, t8944)
}
