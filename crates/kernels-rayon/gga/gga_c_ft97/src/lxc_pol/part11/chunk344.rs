//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 344/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk344(t1751: f64, t409: f64, t1300: f64, t1596: f64, t1599: f64, t1603: f64, t1605: f64, t1617: f64, t1621: f64, t1624: f64, t1626: f64, t1633: f64, t1657: f64, t1660: f64, t1665: f64, t1669: f64, t1671: f64, t1683: f64, t1687: f64, t1698: f64, t1704: f64, t1713: f64, t372: f64, t399: f64, t403: f64, t64: f64, t79: f64) -> (f64, f64) {
    let t1752 = t409 * t1751;
    let t1754 = 0.67598802253579164263e-4_f64 * t1596 * t1599 - 0.46509801892875584e-1_f64 * t1603 * t1605 - 0.13784064983740990796e-3_f64 * t1617 * t1621 + 0.23254900946437792e-1_f64 * t1624 * t1626 + 0.23254900946437792e-2_f64 * t372 * t1633 - 0.11627450473218896e-1_f64 * t372 * t1657 + 0.19365723406274399941e-3_f64 * t372 * t1660 + 2.0_f64 * t1665 + 0.2370952259137005195e-1_f64 * t403 * t399 - 4.0_f64 * t1669 * t1671 + 2.0_f64 * t1687 + 0.14053536537767171586e-3_f64 * t79 * t1698 - 0.11854761295685025975e-1_f64 * t1300 * t1704 - 0.37540077436335915588e-1_f64 * t79 * t1683 + 2.0_f64 * t64 * t1713 - t64 * t1752;
    (t1752, t1754)
}
