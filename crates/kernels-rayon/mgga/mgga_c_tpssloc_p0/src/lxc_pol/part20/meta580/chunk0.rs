//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2146/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2146(t1022: f64, t3120: f64, t2250: f64, t360: f64, t1036: f64, t10367: f64, t1032: f64, t10375: f64, t370: f64, t374: f64, t376: f64, t9697: f64) -> (f64, f64, f64, f64, f64) {
    let t43235 = t3120 * t1022;
    let t43240 = t2250 * t1022;
    let t43241 = t43240 * t360;
    let t43246 = t10367 * t1036;
    let t43248 = t1032 * t10375;
    let t43253 = 7.0_f64 / 31104.0_f64 * t370 * t374 * t9697 * t376;
    (t43235, t43241, t43246, t43248, t43253)
}
