//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 976/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk976(t15125: f64, t312: f64, t1240: f64, t2842: f64, t2844: f64, t10688: f64, t4181: f64, t4239: f64, t870: f64, t875: f64, t2801: f64, t4246: f64) -> (f64, f64, f64, f64, f64) {
    let t15126 = t15125 * t312;
    let t15128 = t1240 * t2842;
    let t15129 = t15128 * t2844;
    let t15131 = t10688 * t4181;
    let t15133 = t4239 * t870;
    let t15134 = t15133 * t875;
    let t15136 = t4246 * t2801;
    (t15126, t15129, t15131, t15134, t15136)
}
