//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1052/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1052(t1017: f64, t20027: f64, t4462: f64, t4668: f64, t4454: f64, t4714: f64, t4458: f64, t20023: f64, t2102: f64, t3499: f64, t3506: f64, t40437: f64, t40466: f64, t462: f64, t49782: f64, t78068: f64, t78070: f64, t78073: f64, t85456: f64, t85491: f64, t86610: f64, t9192: f64, t9217: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86614 = t20027 * t1017;
    let t86618 = t4462 * t4668;
    let t86622 = t4454 * t4714;
    let t86626 = t4454 * t4668;
    let t86630 = t4458 * t4714;
    let t86637 = t20023 * t1017;
    let t86648 = 2.0_f64 * t462 * t2102 * t86610 - 16.0_f64 / 3.0_f64 * t462 * t9192 * t86614 - 4.0_f64 * t462 * t9217 * t86618 + 4.0_f64 / 3.0_f64 * t462 * t9192 * t86622 - 8.0_f64 / 3.0_f64 * t462 * t40466 * t86626 - 4.0_f64 * t462 * t2102 * t86630 + 8.0_f64 / 3.0_f64 * t462 * t3506 * t85456 + 40.0_f64 / 27.0_f64 * t462 * t40437 * t86637 + 8.0_f64 * t462 * t3499 * t85491 - 8.0_f64 / 9.0_f64 * t78068 + 8.0_f64 / 3.0_f64 * t78070 + 8.0_f64 / 3.0_f64 * t78073 + 112.0_f64 / 81.0_f64 * t49782;
    (t86614, t86618, t86622, t86626, t86630, t86637, t86648)
}
