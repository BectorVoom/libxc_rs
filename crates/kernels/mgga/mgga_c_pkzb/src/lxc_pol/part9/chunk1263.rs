//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1263/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1263<F: Float>(t3052: F, t6158: F, t2209: F, t836: F, t7996: F, t7999: F, t18427: F, t18430: F, t18433: F, t18440: F, t18443: F, t18445: F, t18448: F, t22190: F, t22193: F, t22196: F, t22199: F, t22202: F, t22205: F, t22207: F, t22209: F, t22215: F) -> (F, F, F, F) {
    let t22217 = t3052 * t6158;
    let t22219 = t2209 * t836;
    let t22220 = t7996 * t22219;
    let t22222 = t7999 * t22219;
    let t22225 = F::cast_from(0.58258125e1_f64) * t22190 - F::cast_from(0.1237865625e0_f64) * t22193 - F::cast_from(0.485484375e1_f64) * t22196 + F::cast_from(0.6189328125e-1_f64) * t22199 - F::cast_from(0.3883875e1_f64) * t22202 + F::cast_from(0.247573125e0_f64) * t22205 - F::cast_from(0.3883875e1_f64) * t22207 - F::cast_from(0.1294625e1_f64) * t22209 + t18440 - F::cast_from(0.28179666666666666667e1_f64) * t18427 + F::cast_from(0.12077e1_f64) * t18430 - F::cast_from(0.301925e0_f64) * t18433 + t18443 + F::cast_from(0.82785e0_f64) * t18448 + F::cast_from(0.247573125e0_f64) * t22215 + F::cast_from(0.82524375e-1_f64) * t22217 + F::cast_from(0.58258125e1_f64) * t22220 - F::cast_from(0.1237865625e0_f64) * t22222 - F::cast_from(0.22076e1_f64) * t18445;
    (t22217, t22220, t22222, t22225)
}
