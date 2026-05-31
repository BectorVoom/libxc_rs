//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 994/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk994<F: Float>(t11789: F, t11860: F, t11921: F, t12000: F, t10099: F, t1096: F, t11706: F, t11708: F, t11718: F, t11721: F, t11725: F, t2464: F, t2469: F, t3265: F, t3268: F, t338: F, t3449: F, t3746: F, t3795: F, t7056: F, t7063: F, t884: F, t9375: F, t972: F) -> (F, F) {
    let t12002 = t11789 + t11860 + t11921 + t12000;
    let t12004 = F::cast_from(4.0_f64) * t10099 * t3268 - F::cast_from(2.0_f64) * t1096 * t9375 + t11706 * t338 - t11708 * t972 - F::cast_from(6.0_f64) * t11718 * t7063 + F::cast_from(4.0_f64) * t11721 * t2469 + F::cast_from(2.0_f64) * t11725 * t2469 - t12002 * t884 - t2464 * t3795 - F::cast_from(2.0_f64) * t3265 * t3449 + F::cast_from(2.0_f64) * t3746 * t7056;
    (t12002, t12004)
}
