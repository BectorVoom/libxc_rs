//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1235/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1235(t4635: f64, t934: f64, t2924: f64, t2848: f64, t2930: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t1614: f64, t945: f64) -> (f64, f64, f64, f64) {
    let t4636 = t4635 * t934;
    let t4638 = 0.16081979498692535067e2_f64 * t2924 * t4636;
    let t4644 = t2930 + 0.57077777777777777777e-2_f64 * t2848 + 0.57077777777777777777e-2_f64 * t4571 - 0.11415555555555555555e-1_f64 * t4576 + 0.34246666666666666666e-1_f64 * t4581 - 0.17123333333333333333e-1_f64 * t4585;
    let t4647 = t1614 * t945;
    (t4636, t4638, t4644, t4647)
}
