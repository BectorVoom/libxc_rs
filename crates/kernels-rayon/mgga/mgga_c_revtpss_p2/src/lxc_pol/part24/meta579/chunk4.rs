//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1789/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1789(t58777: f64, t70942: f64, t83699: f64, t83719: f64, t83731: f64, t83735: f64, t83748: f64, t83751: f64, t83758: f64, t83783: f64, t83798: f64, t6573: f64, t6587: f64) -> (f64, f64) {
    let t91260 = -11.0_f64 / 81.0_f64 * t70942 + t83699 / 27.0_f64 + t83719 / 54.0_f64 - 0.57927562257303111285e-1_f64 * t83731 - 0.57165357490759649296e-3_f64 * t83735 - 0.17149607247227894789e-2_f64 * t83748 + 0.18292914397043087775e-1_f64 * t83751 - 0.16937883700965822013e-3_f64 * t58777 + 0.22866142996303859718e-2_f64 * t83758 - 0.22866142996303859718e-2_f64 * t83783 + 0.34299214494455789578e-2_f64 * t83798;
    let t91272 = t6573 * t6587;
    (t91260, t91272)
}
