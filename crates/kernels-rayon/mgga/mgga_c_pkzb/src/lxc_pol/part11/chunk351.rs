//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 351/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk351(t1173: f64, t1187: f64, t1213: f64, t1215: f64, t1219: f64, t1259: f64, t135: f64, t273: f64, t957: f64) -> f64 {
    let t1263 = t1259 * t135 * t273 * t957 - t1173 + t1187 + t1213 + t1215 - t1219;
    t1263
}
