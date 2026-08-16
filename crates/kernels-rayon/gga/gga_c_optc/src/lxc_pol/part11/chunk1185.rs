//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1185/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1185(t17728: f64, t3132: f64, t45304: f64, t1506: f64, t3107: f64, t1111: f64, t17897: f64, t24: f64, t11940: f64, t15271: f64, t17936: f64, t26134: f64, t3116: f64) -> (f64, f64, f64, f64, f64) {
    let t54109 = t3132 * t45304 * t17728;
    let t54111 = t3107 * t1506;
    let t54120 = t1111 * t24 * t17897;
    let t54141 = t11940 * t15271;
    let t54174 = t3116 * t26134 * t17936;
    (t54109, t54111, t54120, t54141, t54174)
}
