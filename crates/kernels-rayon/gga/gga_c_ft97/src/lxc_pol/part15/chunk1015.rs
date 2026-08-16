//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1015/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1015(t20213: f64, t2992: f64, t11468: f64, t11906: f64, t11987: f64, t11988: f64, t12020: f64, t12045: f64, t16030: f64, t16052: f64, t1901: f64, t1902: f64, t1909: f64, t20204: f64, t20209: f64, t20239: f64, t39026: f64, t4431: f64, t4458: f64, t4551: f64, t47222: f64, t75136: f64, t75138: f64, t75188: f64, t8518: f64, t85401: f64, t85740: f64, t925: f64) -> (f64, f64) {
    let t85783 = t2992 * t20213;
    let t85789 = 8.0_f64 / 9.0_f64 * t1901 * t47222 * t20209 + 8.0_f64 / 9.0_f64 * t1901 * t16030 * t20204 - 16.0_f64 / 9.0_f64 * t1901 * t8518 * t12020 * t85740 + 8.0_f64 / 3.0_f64 * t1901 * t39026 * t75188 * t925 - 8.0_f64 / 3.0_f64 * t1901 * t11906 * t20239 - 4.0_f64 / 3.0_f64 * t1901 * t1909 * t12045 * t4431 * t4551 - 4.0_f64 / 3.0_f64 * t1901 * t1902 * t16052 * t4458 - 20.0_f64 / 27.0_f64 * t1901 * t11987 * t11988 * t85401 - 8.0_f64 / 3.0_f64 * t1901 * t11468 * t85783 + 4.0_f64 / 27.0_f64 * t75136 + 8.0_f64 / 27.0_f64 * t75138;
    (t85783, t85789)
}
