//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1417/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1417(t1535: f64, t5471: f64, t2922: f64, t30599: f64, t26728: f64, t26973: f64, t26976: f64, t2868: f64, t2881: f64, t30603: f64, t30604: f64, t30607: f64, t30611: f64, t30617: f64, t30642: f64, t9646: f64, t9650: f64, t9657: f64, t9660: f64) -> (f64, f64, f64) {
    let t30657 = t5471 * t1535;
    let t30665 = t2922 * t30657;
    let t30670 = t2922 * t30599;
    let t30681 = -256.0_f64 / 27.0_f64 * t2881 * t30611 * t9657 - 1600.0_f64 / 27.0_f64 * t26973 * t30617 - 128.0_f64 / 9.0_f64 * t30604 * t9660 - 1280.0_f64 / 27.0_f64 * t2868 * t30657 * t9646 + 8000.0_f64 / 27.0_f64 * t26976 * t30642 - 640.0_f64 / 9.0_f64 * t30607 * t9650 + 512.0_f64 / 9.0_f64 * t30665 * t9657 - 3200.0_f64 / 9.0_f64 * t26728 * t30617 + 256.0_f64 / 3.0_f64 * t30670 * t9660 + 512.0_f64 / 9.0_f64 * t2922 * t30611 * t9646 + 3200.0_f64 / 9.0_f64 * t26728 * t30642 + 256.0_f64 / 3.0_f64 * t2922 * t30603 * t9650;
    (t30657, t30670, t30681)
}
