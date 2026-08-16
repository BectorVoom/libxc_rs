//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 357/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk357(t1255: f64, t942: f64, t1246: f64, t411: f64, t415: f64, t1173: f64, t1187: f64, t1213: f64, t1215: f64, t1219: f64, t135: f64, t273: f64, t957: f64) -> (f64, f64, f64) {
    let t1256 = t942 * t1255;
    let t1259 = 0.65854491829355115987e0_f64 * t1246 * t415 - 0.65854491829355115987e0_f64 * t411 * t1256;
    let t1263 = t1259 * t135 * t273 * t957 - t1173 + t1187 + t1213 + t1215 - t1219;
    (t1256, t1259, t1263)
}
