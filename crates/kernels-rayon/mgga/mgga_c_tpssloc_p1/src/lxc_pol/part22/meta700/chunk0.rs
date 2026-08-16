//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2284/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2284(t1174: f64, t18206: f64, t44562: f64, t1227: f64, t13969: f64, t18958: f64, t248: f64, t45046: f64, t5971: f64, t15643: f64, t5005: f64, t1009: f64, t18571: f64) -> (f64, f64, f64, f64, f64) {
    let t65914 = t1174 * t44562 * t18206;
    let t65920 = t1227 * t13969 * t18958;
    let t65935 = t1227 * t248 * t45046 * t5971;
    let t65952 = t5005 * t15643;
    let t65955 = t18571 * t1009;
    (t65914, t65920, t65935, t65952, t65955)
}
