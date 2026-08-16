//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 992/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk992(t4589: f64, t1852: f64, t20044: f64, t920: f64, t4417: f64, t4551: f64, t11472: f64, t11854: f64, t16246: f64, t1871: f64, t1901: f64, t20045: f64, t20177: f64, t20218: f64, t20268: f64, t20279: f64, t20395: f64, t2992: f64, t3238: f64, t39120: f64, t4436: f64, t446: f64, t447: f64, t4495: f64, t452: f64, t4572: f64, t488: f64, t74899: f64, t925: f64, t942: f64, t986: f64) -> (f64, f64, f64, f64) {
    let t85315 = t4589 * t4589;
    let t85316 = t1852 * t85315;
    let t85320 = t20044 * t920;
    let t85325 = t4417 * t4551;
    let t85380 = -4.0_f64 * t446 * t1871 * t488 * t4436 * t4589 - 4.0_f64 / 9.0_f64 * t446 * t447 * t986 * t20045 + 4.0_f64 * t446 * t452 * t16246 * t4572 - 8.0_f64 * t446 * t1871 * t3238 * t20177 - 8.0_f64 / 3.0_f64 * t74899 + 8.0_f64 / 3.0_f64 * t1901 * t39120 * t20177 * t925 - 8.0_f64 / 3.0_f64 * t1901 * t11854 * t20268 * t925 - 8.0_f64 / 3.0_f64 * t1901 * t11472 * t2992 * t20218 + 4.0_f64 * t446 * t452 * t3238 * t20279 + 2.0_f64 * t446 * t452 * t488 * t4495 * t4589 + 4.0_f64 / 3.0_f64 * t446 * t452 * t488 * t942 * t20395;
    (t85316, t85320, t85325, t85380)
}
