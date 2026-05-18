//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 866/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk866<F: Float>(t1372: F, t26004: F, t1389: F, t7269: F, t2736: F, t2689: F, t7256: F, t2018: F, t3951: F, t807: F, t25240: F, t3964: F) -> (F, F, F, F, F) {
    let t26005 = t26004 * t1372;
    let t26009 = t7269 * t1389;
    let t26010 = t2736 * t26009;
    let t26011 = F::new(0.50820002809285328225e-5) * t26010;
    let t26012 = t2689 * t7256;
    let t26013 = F::new(0.15244095330869239812e-3) * t26012;
    let t26014 = t2018 * t3951;
    let t26015 = t807 * t26014;
    let t26021 = t3964 * t25240 * t1389;
    (t26005, t26011, t26013, t26015, t26021)
}
