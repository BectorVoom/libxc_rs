//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1879/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1879<F: Float>(t26009: F, t2736: F, t2689: F, t7256: F, t2018: F, t3951: F, t807: F, t1941: F, t550: F, t3946: F, t1389: F, t25240: F, t3964: F) -> (F, F, F, F, F, F, F) {
    let t26010 = t2736 * t26009;
    let t26011 = F::cast_from(0.50820002809285328225e-5_f64) * t26010;
    let t26012 = t2689 * t7256;
    let t26013 = F::cast_from(0.15244095330869239812e-3_f64) * t26012;
    let t26014 = t2018 * t3951;
    let t26015 = t807 * t26014;
    let t26016 = F::cast_from(0.11433071498151929859e-3_f64) * t26015;
    let t26017 = t1941 * t550;
    let t26018 = t26017 * t3946;
    let t26021 = t3964 * t25240 * t1389;
    (t26011, t26013, t26014, t26015, t26016, t26018, t26021)
}
