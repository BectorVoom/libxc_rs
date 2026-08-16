//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1369/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1369<F: Float>(t17934: F, t5808: F, t10523: F, t76637: F, t951: F, t959: F, t21095: F, t4483: F, t48103: F, t68442: F, t68444: F, t68446: F, t68448: F, t68452: F, t68454: F, t68494: F, t68498: F, t68500: F, t77028: F, t77030: F, t77032: F, t77034: F) -> (F, F, F, F) {
    let t77153 = F::cast_from(0.35089341735807877242e1_f64) * t17934 * t5808;
    let t77157 = F::cast_from(0.14035736694323150897e2_f64) * t959 * t10523 * t76637 * t951;
    let t77159 = F::cast_from(0.4155806185363551302e3_f64) * t4483 * t21095;
    let t77174 = F::cast_from(0.24154e1_f64) * t68442 + F::cast_from(0.40256666666666666668e0_f64) * t68444 + F::cast_from(0.44729629629629629629e0_f64) * t68446 - F::cast_from(0.16102666666666666667e1_f64) * t68448 - F::cast_from(0.132456e1_f64) * t68452 + F::cast_from(0.22076e0_f64) * t68454 + F::cast_from(0.98115555555555555556e0_f64) * t48103 + F::cast_from(0.80513333333333333333e0_f64) * t68494 - F::cast_from(0.24154e1_f64) * t68498 + F::cast_from(0.11651625e2_f64) * t77028 - F::cast_from(0.51785e1_f64) * t77030 - F::cast_from(0.247573125e0_f64) * t77032 + F::cast_from(0.3300975e0_f64) * t77034 + F::cast_from(0.98115555555555555555e-1_f64) * t68500;
    (t77153, t77157, t77159, t77174)
}
