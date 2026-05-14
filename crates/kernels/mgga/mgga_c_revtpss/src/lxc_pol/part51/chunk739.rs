//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 739/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk739<F: Float>(t26010: F, t2689: F, t7256: F, t2018: F, t3951: F, t807: F, t1389: F, t25240: F, t3964: F, t7262: F, t820: F, t843: F, t1401: F, t241: F, t3920: F, t7246: F) -> (F, F, F, F, F, F, F, F) {
    let t26011 = 0.50820002809285328225e-5 * t26010;
    let t26012 = t2689 * t7256;
    let t26013 = 0.15244095330869239812e-3 * t26012;
    let t26014 = t2018 * t3951;
    let t26015 = t807 * t26014;
    let t26021 = t3964 * t25240 * t1389;
    let t26022 = 0.90357964994909313586e-5 * t26021;
    let t26024 = t820 * t7262 * t843;
    let t26025 = t26024 * t1401;
    let t26028 = t820 * t7262 * t241;
    let t26040 = 0.13009920719177044025e-1 * t7246 * t3920;
    (t26011, t26013, t26015, t26022, t26024, t26025, t26028, t26040)
}
