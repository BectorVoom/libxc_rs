//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 331/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk331(t1116: f64, t730: f64, t1066: f64, t154: f64, t742: f64, t1072: f64, t1086: f64, t1112: f64, t1114: f64) -> (f64, f64, f64) {
    let t1118 = 0.5848223622634646207e0_f64 * t730 * t1116;
    let t1120 = t154 * t742 * t1066;
    let t1123 = -t1072 + t1086 + t1112 + t1114 - t1118;
    (t1118, t1120, t1123)
}
