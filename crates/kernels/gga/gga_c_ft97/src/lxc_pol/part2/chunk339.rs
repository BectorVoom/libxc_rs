//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 339/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk339<F: Float>(t1751: F, t409: F, t1300: F, t1596: F, t1599: F, t1603: F, t1605: F, t1617: F, t1621: F, t1624: F, t1626: F, t1633: F, t1657: F, t1660: F, t1665: F, t1669: F, t1671: F, t1683: F, t1687: F, t1698: F, t1704: F, t1713: F, t372: F, t399: F, t403: F, t64: F, t79: F) -> F {
    let t1752 = t409 * t1751;
    let t1754 = F::cast_from(0.67598802253579164263e-4_f64) * t1596 * t1599 - F::cast_from(0.46509801892875584e-1_f64) * t1603 * t1605 - F::cast_from(0.13784064983740990796e-3_f64) * t1617 * t1621 + F::cast_from(0.23254900946437792e-1_f64) * t1624 * t1626 + F::cast_from(0.23254900946437792e-2_f64) * t372 * t1633 - F::cast_from(0.11627450473218896e-1_f64) * t372 * t1657 + F::cast_from(0.19365723406274399941e-3_f64) * t372 * t1660 + F::new(2.0) * t1665 + F::cast_from(0.2370952259137005195e-1_f64) * t403 * t399 - F::new(4.0) * t1669 * t1671 + F::new(2.0) * t1687 + F::cast_from(0.14053536537767171586e-3_f64) * t79 * t1698 - F::cast_from(0.11854761295685025975e-1_f64) * t1300 * t1704 - F::cast_from(0.37540077436335915588e-1_f64) * t79 * t1683 + F::new(2.0) * t64 * t1713 - t64 * t1752;
    t1754
}
