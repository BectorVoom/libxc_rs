//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1167/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1167(t27: f64, t28676: f64, t19530: f64, t23870: f64, t2504: f64, t28696: f64, t28700: f64, t28704: f64, t28707: f64, t28710: f64, t28714: f64, t3347: f64, t38: f64, t6738: f64, t8646: f64, t8650: f64, t8654: f64, t8658: f64, t991: f64) -> (f64, f64) {
    let t28718 = -t27 * t28676;
    let t28721 = -200.0_f64 / 9.0_f64 * t3347 * t2504 + 50.0_f64 / 27.0_f64 * t991 * t8646 + 100.0_f64 / 9.0_f64 * t23870 * t8650 - 50.0_f64 / 9.0_f64 * t991 * t8654 - 25.0_f64 / 3.0_f64 * t991 * t8658 + 40.0_f64 / 81.0_f64 * t38 * t28696 + 10.0_f64 / 9.0_f64 * t19530 * t28700 - 10.0_f64 / 9.0_f64 * t19530 * t28704 - 10.0_f64 / 3.0_f64 * t6738 * t28707 + 10.0_f64 / 3.0_f64 * t38 * t28710 + 10.0_f64 / 9.0_f64 * t38 * t28714 + 5.0_f64 / 3.0_f64 * t38 * t28718;
    (t28718, t28721)
}
