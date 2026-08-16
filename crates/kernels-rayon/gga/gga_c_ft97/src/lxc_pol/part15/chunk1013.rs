//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1013/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1013(t79: f64, t85573: f64, t85602: f64, t85644: f64, t85679: f64, t4495: f64, t4436: f64, t110: f64, t12020: f64, t16076: f64, t16228: f64, t1871: f64, t1901: f64, t1909: f64, t3193: f64, t38921: f64, t39230: f64, t4454: f64, t446: f64, t4462: f64, t452: f64, t59937: f64, t60100: f64, t75034: f64, t8217: f64, t85325: f64, t85401: f64) -> (f64, f64, f64, f64) {
    let t80 = 0.1e-59_f64 < t79;
    let t85682 = piecewise3(t80, t85573 + t85602 + t85644 + t85679, 0.0_f64);
    let t85687 = t4495 * t4495;
    let t85692 = t4436 * t4436;
    let t85723 = -t446 * t452 * t110 * t85682 / 3.0_f64 + 2.0_f64 * t446 * t1871 * t110 * t85687 + 8.0_f64 * t446 * t38921 * t110 * t85692 + 16.0_f64 / 9.0_f64 * t59937 - 8.0_f64 / 9.0_f64 * t1901 * t3193 * t60100 * t85325 + 8.0_f64 / 3.0_f64 * t1901 * t3193 * t12020 * t85401 - 4.0_f64 / 3.0_f64 * t1901 * t8217 * t16228 * t4462 - 8.0_f64 / 9.0_f64 * t1901 * t39230 * t16228 * t4454 + 4.0_f64 / 9.0_f64 * t1901 * t3193 * t16076 * t4454 + 2.0_f64 / 3.0_f64 * t1901 * t1909 * t16076 * t4462 + 8.0_f64 / 3.0_f64 * t75034;
    (t85682, t85687, t85692, t85723)
}
