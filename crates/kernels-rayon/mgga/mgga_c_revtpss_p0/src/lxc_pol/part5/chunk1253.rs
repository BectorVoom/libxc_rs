//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1253/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1253(t19971: f64, t4893: f64, t3117: f64, t11922: f64, t6272: f64, t3115: f64, t1668: f64, t3181: f64, t372: f64, t1045: f64, t4574: f64, t12131: f64, t6266: f64) -> (f64, f64, f64, f64) {
    let t19972 = t4893 * t19971;
    let t19973 = t3117 * t19972;
    let t19976 = t11922 * t6272;
    let t19977 = t3115 * t19976;
    let t19979 = t3181 * t1668;
    let t19980 = t372 * t19979;
    let t19981 = t1045 * t4574;
    let t19982 = t19980 * t19981;
    let t19985 = t12131 * t6266;
    (t19973, t19977, t19982, t19985)
}
