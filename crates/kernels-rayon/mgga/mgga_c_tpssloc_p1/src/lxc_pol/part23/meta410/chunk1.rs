//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1226/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1226(t15568: f64, t5064: f64, t1227: f64, t248: f64, t45046: f64, t5971: f64, t3032: f64, t65253: f64, t3505: f64, t3514: f64, t1174: f64, t6187: f64, t698: f64) -> (f64, f64, f64, f64, f64) {
    let t65884 = t5064 * t15568;
    let t65935 = t1227 * t248 * t45046 * t5971;
    let t65962 = t65253 * t3032;
    let t65963 = t65962 * t3505;
    let t65966 = t65962 * t3514;
    let t66015 = t1174 * t698 * t6187;
    (t65884, t65935, t65963, t65966, t66015)
}
