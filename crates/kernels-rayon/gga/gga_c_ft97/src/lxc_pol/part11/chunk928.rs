//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 928/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk928(t38052: f64, t82: f64, t358: f64, t492: f64, t1820: f64, t363: f64, t110: f64, t11854: f64, t12045: f64, t1580: f64, t1647: f64, t1853: f64, t1866: f64, t1901: f64, t1909: f64, t1910: f64, t3194: f64, t379: f64, t38053: f64, t38057: f64, t38071: f64, t38942: f64, t39228: f64, t39230: f64, t446: f64, t447: f64, t499: f64, t7955: f64, t8367: f64, t8417: f64, t8419: f64, t8577: f64) -> f64 {
    let t39243 = t38052 * t82;
    let t39252 = t492 * t358;
    let t39253 = t363 * t1820;
    let t39267 = 2.0_f64 / 3.0_f64 * t1901 * t1909 * t1910 * t1580 * t1820 - 4.0_f64 / 3.0_f64 * t1901 * t1909 * t12045 * t1580 * t1853 + 16.0_f64 / 27.0_f64 * t39228 - 8.0_f64 / 9.0_f64 * t1901 * t39230 * t3194 * t38942 - 40.0_f64 / 81.0_f64 * t446 * t8577 * t499 * t7955 - t446 * t447 * t110 * t38057 / 9.0_f64 - 80.0_f64 / 243.0_f64 * t446 * t39243 * t110 * t38053 - 8.0_f64 / 3.0_f64 * t446 * t1866 * t110 * t38071 - 8.0_f64 / 3.0_f64 * t1901 * t11854 * t39252 * t39253 + 8.0_f64 / 3.0_f64 * t1901 * t1909 * t8417 * t8419 * t379 + 8.0_f64 / 3.0_f64 * t1901 * t1909 * t8367 * t1647;
    t39267
}
