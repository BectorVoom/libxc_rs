//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 977/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk977(t10666: f64, t1248: f64, t2749: f64, t4299: f64, t15125: f64, t295: f64, t312: f64, t1250: f64, t8232: f64, t1091: f64, t2894: f64, t835: f64) -> (f64, f64, f64, f64, f64) {
    let t15138 = t10666 * t1248;
    let t15140 = t2749 * t4299;
    let t15143 = t295 * t15125 * t312;
    let t15147 = t8232 * t1250;
    let t15150 = t835 * t2894 * t1091;
    (t15138, t15140, t15143, t15147, t15150)
}
