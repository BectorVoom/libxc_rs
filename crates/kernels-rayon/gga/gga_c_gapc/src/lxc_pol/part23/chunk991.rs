//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 991/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk991(t11789: f64, t11860: f64, t11921: f64, t12000: f64, t10099: f64, t1096: f64, t11706: f64, t11708: f64, t11718: f64, t11721: f64, t11725: f64, t2464: f64, t2469: f64, t3265: f64, t3268: f64, t338: f64, t3449: f64, t3746: f64, t3795: f64, t7056: f64, t7063: f64, t884: f64, t9375: f64, t972: f64) -> (f64, f64) {
    let t12002 = t11789 + t11860 + t11921 + t12000;
    let t12004 = 4.0_f64 * t10099 * t3268 - 2.0_f64 * t1096 * t9375 + t11706 * t338 - t11708 * t972 - 6.0_f64 * t11718 * t7063 + 4.0_f64 * t11721 * t2469 + 2.0_f64 * t11725 * t2469 - t12002 * t884 - t2464 * t3795 - 2.0_f64 * t3265 * t3449 + 2.0_f64 * t3746 * t7056;
    (t12002, t12004)
}
