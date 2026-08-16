//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1016/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1016(t10969: f64, t20268: f64, t20213: f64, t2983: f64, t11490: f64, t11810: f64, t11811: f64, t11902: f64, t11906: f64, t16145: f64, t1901: f64, t1902: f64, t20113: f64, t20191: f64, t20214: f64, t20219: f64, t20438: f64, t446: f64, t4495: f64, t452: f64, t4623: f64, t47273: f64, t60309: f64, t75370: f64, t75372: f64, t75678: f64, t8411: f64, t925: f64, t986: f64) -> (f64, f64, f64) {
    let t85797 = t10969 * t20268;
    let t85825 = t2983 * t20213;
    let t85862 = -8.0_f64 * t1901 * t11490 * t16145 * t20268 - 8.0_f64 * t1901 * t11810 * t11811 * t20191 - 8.0_f64 / 3.0_f64 * t1901 * t47273 * t20438 + 16.0_f64 / 9.0_f64 * t60309 - 8.0_f64 * t446 * t8411 * t986 * t20113 - 2.0_f64 * t446 * t452 * t4623 * t4495 + 4.0_f64 / 9.0_f64 * t1901 * t1902 * t75678 * t925 + 4.0_f64 / 3.0_f64 * t1901 * t11902 * t20214 + 4.0_f64 / 3.0_f64 * t1901 * t11906 * t20219 + 4.0_f64 / 9.0_f64 * t75370 + 8.0_f64 / 3.0_f64 * t75372;
    (t85797, t85825, t85862)
}
