//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 923/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk923(t10760: f64, t13733: f64, t1426: f64, t17276: f64, t17284: f64, t17287: f64, t17312: f64, t2301: f64, t350: f64, t4009: f64, t4835: f64, t4846: f64, t8345: f64, t974: f64) -> f64 {
    let t17314 = 6.0_f64 * t10760 * t4835 - 3.0_f64 * t13733 * t1426 + t17276 * t350 - 6.0_f64 * t8345 * t17284 + 6.0_f64 * t2301 * t17287 - t974 * t17312 - 3.0_f64 * t4009 * t4846;
    t17314
}
