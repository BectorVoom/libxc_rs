//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1369/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1369(t17934: f64, t5808: f64, t10523: f64, t76637: f64, t951: f64, t959: f64, t21095: f64, t4483: f64, t48103: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68452: f64, t68454: f64, t68494: f64, t68498: f64, t68500: f64, t77028: f64, t77030: f64, t77032: f64, t77034: f64) -> (f64, f64, f64, f64) {
    let t77153 = 0.35089341735807877242e1_f64 * t17934 * t5808;
    let t77157 = 0.14035736694323150897e2_f64 * t959 * t10523 * t76637 * t951;
    let t77159 = 0.4155806185363551302e3_f64 * t4483 * t21095;
    let t77174 = 0.24154e1_f64 * t68442 + 0.40256666666666666668e0_f64 * t68444 + 0.44729629629629629629e0_f64 * t68446 - 0.16102666666666666667e1_f64 * t68448 - 0.132456e1_f64 * t68452 + 0.22076e0_f64 * t68454 + 0.98115555555555555556e0_f64 * t48103 + 0.80513333333333333333e0_f64 * t68494 - 0.24154e1_f64 * t68498 + 0.11651625e2_f64 * t77028 - 0.51785e1_f64 * t77030 - 0.247573125e0_f64 * t77032 + 0.3300975e0_f64 * t77034 + 0.98115555555555555555e-1_f64 * t68500;
    (t77153, t77157, t77159, t77174)
}
