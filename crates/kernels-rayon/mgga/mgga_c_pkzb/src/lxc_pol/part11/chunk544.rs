//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 544/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk544(t2873: f64, t2874: f64, t730: f64, t1066: f64, t154: f64, t2048: f64, t276: f64, t153: f64, t275: f64) -> (f64, f64, f64, f64, f64) {
    let t2875 = t2873 * t2874;
    let t2877 = 0.17315859105681463759e2_f64 * t730 * t2875;
    let t2883 = t154 * t2048 * t1066;
    let t2884 = t276 * t2883;
    let t2886 = t275 * t153;
    (t2875, t2877, t2883, t2884, t2886)
}
