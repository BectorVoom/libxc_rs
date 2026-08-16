//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 944/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk944(t18587: f64, t241: f64, t258: f64, t1882: f64, t5153: f64, t2574: f64, t4934: f64, t773: f64, t11593: f64, t14126: f64, t14138: f64, t18516: f64, t18521: f64, t18526: f64, t18529: f64, t18534: f64, t18538: f64, t18540: f64, t18542: f64, t18544: f64, t1901: f64, t193: f64, t446: f64, t89: f64, t9982: f64) -> f64 {
    let t18589 = t241 * t18587 * t258;
    let t18593 = t1882 * t5153;
    let t18596 = t2574 * t773 * t4934;
    let t18599 = -2.0_f64 / 3.0_f64 * t1901 * t18516 - 4.0_f64 / 9.0_f64 * t11593 * t18521 - 4.0_f64 / 9.0_f64 * t11593 * t18526 + 2.0_f64 / 9.0_f64 * t1901 * t18529 + 2.0_f64 / 9.0_f64 * t1901 * t18534 - t9982 - t14126 - 4.0_f64 / 27.0_f64 * t14138 + 2.0_f64 / 81.0_f64 * t18538 + t18540 / 27.0_f64 + 2.0_f64 / 27.0_f64 * t18542 - 2.0_f64 / 9.0_f64 * t18544 + t89 * t193 * t18589 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t18593 + 2.0_f64 / 3.0_f64 * t446 * t18596;
    t18599
}
