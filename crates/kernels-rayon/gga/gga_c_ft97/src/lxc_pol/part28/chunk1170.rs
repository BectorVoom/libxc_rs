//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1170/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1170(t104462: f64, t5956: f64, t11593: f64, t140325: f64, t144: f64, t148678: f64, t148960: f64, t148964: f64, t148966: f64, t167: f64, t1901: f64, t2185: f64, t2210: f64, t23478: f64, t3052: f64, t33055: f64, t34918: f64, t446: f64, t574: f64, t5975: f64, t605: f64, t609: f64, t6630: f64, t6695: f64, t6699: f64, t95767: f64) -> (f64, f64) {
    let t148977 = t104462 * t5956;
    let t148997 = -t446 * t144 * t148960 / 3.0_f64 + t148964 / 9.0_f64 + t148966 / 9.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t2185 * t5975 * t6630 - 4.0_f64 / 9.0_f64 * t140325 + 2.0_f64 / 3.0_f64 * t446 * t574 * t23478 * t6699 + 4.0_f64 / 3.0_f64 * t446 * t144 * t148977 + 2.0_f64 / 3.0_f64 * t446 * t2185 * t167 * t148678 + t446 * t574 * t605 * t34918 * t609 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t11593 * t2210 * t33055 * t3052 + 2.0_f64 / 9.0_f64 * t1901 * t95767 * t6695;
    (t148977, t148997)
}
