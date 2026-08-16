//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1987/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1987(t3789: f64, t5234: f64, t3798: f64, t1354: f64, t12211: f64, t5223: f64, t1307: f64, t210: f64, t5226: f64, t1810: f64, t3719: f64, t3804: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16285 = t5234 * t3789;
    let t16288 = t5234 * t3798;
    let t16290 = 7.0_f64 / 2304.0_f64 * t16288 * t1354;
    let t16294 = 7.0_f64 / 24.0_f64 * t12211 * t5223;
    let t16296 = t210 * t5226 * t1307;
    let t16300 = t210 * t1810 * t3719;
    let t16305 = t3804 * t820;
    (t16285, t16288, t16290, t16294, t16296, t16300, t16305)
}
