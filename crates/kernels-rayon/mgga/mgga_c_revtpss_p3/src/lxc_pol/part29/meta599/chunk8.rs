//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2047/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2047(t28944: f64, t575: f64, t104071: f64, t104073: f64, t104077: f64, t104079: f64, t104081: f64, t104083: f64, t1456: f64, t1914: f64, t26743: f64, t28993: f64, t5790: f64, t5808: f64, t7542: f64, t7560: f64, t95196: f64, t96633: f64) -> f64 {
    let t104085 = 2.0_f64 * t28944 * t575;
    let t104087 = 2.0_f64 * t1456 * t28993 + t1914 * t26743 + 2.0_f64 * t5790 * t7560 + 2.0_f64 * t5808 * t7542 + t104071 + t104073 + t104077 + t104079 + t104081 + t104083 + t104085 + t95196 + t96633;
    t104087
}
