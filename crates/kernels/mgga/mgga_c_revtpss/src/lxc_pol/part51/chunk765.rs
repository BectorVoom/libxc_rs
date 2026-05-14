//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 765/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk765<F: Float>(t1967: F, t816: F, t1014: F, t65: F, t4579: F, t3252: F, t4574: F, t3204: F, t7131: F, t4817: F, t7132: F, t25517: F, t25543: F, t25551: F, t25557: F, t25560: F, t25564: F, t4783: F, t4788: F, t4839: F) -> (F,) {
    let t27526 = t1967 * t816;
    let t27527 = t65 * t1014;
    let t27528 = t27527 * t4579;
    let t27531 = t65 * t3252;
    let t27532 = t27531 * t4574;
    let t27536 = t3204 * t7131;
    let t27539 = t7132 * t4817;
    let t27541 = 0.28582678745379824648e-3 * t25517 * t4783 + 0.28582678745379824648e-3 * t25517 * t4788 + t25543 / 864.0 + 0.19055119163586549765e-3 * t25551 - 0.15244095330869239812e-2 * t25557 - t27526 * t27528 / 144.0 + t27526 * t27532 / 216.0 - t25560 + 0.28582678745379824648e-3 * t25564 + 0.85748036236139473944e-3 * t27536 * t4839 + 0.19055119163586549765e-3 * t27539;
    (t27541,)
}
