//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1053/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1053(t20660: f64, t925: f64, t4458: f64, t4668: f64, t1017: f64, t20035: f64, t4714: f64, t12823: f64, t2102: f64, t2112: f64, t24: f64, t3499: f64, t3506: f64, t40379: f64, t40425: f64, t462: f64, t62587: f64, t62599: f64, t62629: f64, t62669: f64, t78089: f64, t78091: f64, t85465: f64, t85474: f64, t85483: f64, t92: f64, t9217: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t86661 = t925 * t20660;
    let t86665 = t4458 * t4668;
    let t86669 = t20035 * t1017;
    let t86676 = t4668 * t4668;
    let t86681 = t4714 * t4714;
    let t86686 = 4.0_f64 / 3.0_f64 * t78089 + 8.0_f64 * t78091 - 16.0_f64 / 27.0_f64 * t62587 + 16.0_f64 / 9.0_f64 * t62599 + 16.0_f64 / 9.0_f64 * t62629 - 8.0_f64 / 9.0_f64 * t62669 - 8.0_f64 / 9.0_f64 * t462 * t3499 * t85465 - 20.0_f64 / 9.0_f64 * t462 * t12823 * t85483 + 8.0_f64 * t462 * t40425 * t86661 + 8.0_f64 * t462 * t9217 * t86665 + 8.0_f64 * t462 * t2102 * t86669 - 12.0_f64 * t462 * t3506 * t85474 + 24.0_f64 * t92 * t24 * t40379 * t86676 + 6.0_f64 * t92 * t24 * t2112 * t86681;
    (t86661, t86665, t86669, t86676, t86681, t86686)
}
