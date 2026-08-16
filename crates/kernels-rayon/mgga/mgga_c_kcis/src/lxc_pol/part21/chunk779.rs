//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 779/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk779(t8930: f64, t9004: f64, t2764: f64, t898: f64, t2770: f64, t895: f64, t897: f64, t224: f64, t2772: f64, t906: f64, t2789: f64, t2150: f64, t805: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9005 = t8930 + t9004;
    let t9007 = t2764 * t898;
    let t9010 = t895 * t2770;
    let t9015 = t897 * t897;
    let t9016 = 1.0_f64 / t9015;
    let t9017 = t224 * t9016;
    let t9018 = t2772 * t906;
    let t9021 = t906 * t2789;
    let t9024 = t805 * t2150;
    (t9005, t9007, t9010, t9015, t9016, t9017, t9018, t9021, t9024)
}
