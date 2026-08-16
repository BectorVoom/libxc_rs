//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1447/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1447(t1381: f64, t3699: f64, t1383: f64, t12030: f64, t501: f64, t605: f64, t161: f64, t39048: f64, t12250: f64, t13846: f64, t1841: f64, t1845: f64, t1850: f64, t1854: f64, t1858: f64, t29160: f64, t29162: f64, t29184: f64, t29186: f64, t29210: f64, t29212: f64, t29224: f64, t29226: f64, t29230: f64, t39040: f64, t39149: f64, t5396: f64, t7289: f64, t734: f64) -> (f64, f64, f64) {
    let t39337 = t3699 * t1381;
    let t39339 = 2.0_f64 * t39337 * t1383;
    let t39340 = t12030 * t501;
    let t39342 = 2.0_f64 * t39340 * t605;
    let t39347 = t39048 * t161;
    let t39361 = t29160 - t29162 + t29184 + t29186 - 0.17090058289204942853e-2_f64 * t1841 * t1858 * t13846 * t734 - t29210 - t29212 - t29224 - t29226 + 0.51270174867614828558e-2_f64 * t1841 * t39347 * t1845 - 0.17090058289204942853e-2_f64 * t1850 * t5396 * t39149 - 0.34180116578409885705e-2_f64 * t1841 * t7289 * t39040 + 0.17090058289204942853e-2_f64 * t1850 * t12250 * t161 * t1854 - t29230;
    (t39339, t39342, t39361)
}
