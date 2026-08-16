//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1338/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1338(t10760: f64, t13733: f64, t1426: f64, t17284: f64, t17287: f64, t17312: f64, t2301: f64, t23708: f64, t30408: f64, t350: f64, t39623: f64, t4009: f64, t4835: f64, t4846: f64, t49417: f64, t58056: f64, t58067: f64, t58080: f64, t58093: f64, t58109: f64, t58115: f64, t58132: f64, t58143: f64, t58156: f64, t58169: f64, t8345: f64, t974: f64) -> f64 {
    let t58173 = (t58056 + t58067 + t58080 + t58093) * t350 - 4.0_f64 * t49417 * t1426 + 12.0_f64 * t39623 * t4835 - 6.0_f64 * t13733 * t4846 - 24.0_f64 * t30408 * t17284 + 24.0_f64 * t10760 * t17287 - 4.0_f64 * t4009 * t17312 + 24.0_f64 * t23708 * t58109 - 36.0_f64 * t8345 * t4835 * t4846 + 6.0_f64 * t2301 * t58115 + 8.0_f64 * t2301 * t1426 * t17312 - t974 * (t58132 + t58143 + t58156 + t58169);
    t58173
}
