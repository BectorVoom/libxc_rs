//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 678/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk678(t7609: f64, t826: f64, t2153: f64, t2533: f64, t2538: f64, t113: f64, t805: f64, t774: f64, t808: f64, t153: f64, t740: f64, t2150: f64, t815: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7610 = t7609 * t826;
    let t7611 = t2533 * t2153;
    let t7612 = t2153 * t826;
    let t7613 = t2538 * t7612;
    let t7614 = 2.0_f64 * t7613;
    let t7615 = t805 * t113;
    let t7617 = t113 * t774;
    let t7618 = t808 * t7617;
    let t7620 = t153 * t740;
    let t7622 = t815 * t2150;
    (t7610, t7611, t7612, t7614, t7615, t7617, t7618, t7620, t7622)
}
