//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 327/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk327(t12: f64, t1072: f64, t1086: f64, t1112: f64, t1114: f64, t1118: f64, t1147: f64, t135: f64, t273: f64, t805: f64, t972: f64, t977: f64, t326: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t1151 = t1147 * t135 * t273 * t805 - t1072 + t1086 + t1112 + t1114 - t1118;
    let t1153 = piecewise3(t84, 0.0_f64, t972);
    let t1161 = 1.0_f64 / t977;
    let t1162 = t326 * t1161;
    (t1151, t1153, t1162)
}
