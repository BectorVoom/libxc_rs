//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1269/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1269<F: Float>(t1165: F, t1552: F, t1759: F, t3451: F, t864: F, t3379: F, t6271: F, t1487: F, t407: F, t1173: F, t1180: F, t1181: F, t1532: F, t18027: F, t18031: F, t18035: F, t18037: F, t18041: F, t18045: F, t18047: F, t301: F, t372: F, t5799: F) -> (F, F) {
    let t23429 = t3451 * t1165 * t1552 * t1759 * t864;
    let t23431 = t3379 * t6271;
    let t23445 = t407 * t1487;
    let t23450 = -F::cast_from(0.34299214494455789578e-2_f64) * t18027 - F::cast_from(0.34299214494455789578e-1_f64) * t18031 - F::cast_from(0.17149607247227894789e-2_f64) * t18035 + F::cast_from(0.32012600194825403606e-1_f64) * t18037 - F::cast_from(0.85748036236139473944e-3_f64) * t18041 + F::cast_from(0.17149607247227894789e-2_f64) * t23429 + F::cast_from(0.68598428988911579156e-2_f64) * t23431 - F::cast_from(0.85748036236139473944e-3_f64) * t18045 + F::cast_from(0.16006300097412701803e-1_f64) * t18047 + F::cast_from(0.68598428988911579156e-2_f64) * t1173 * t1181 * t1532 * t5799 * t301 + F::cast_from(0.34299214494455789578e-2_f64) * t1180 * t1181 * t1552 * t5799 * t372 - F::cast_from(0.17149607247227894789e-2_f64) * t1180 * t1181 * t1532 * t23445;
    (t23445, t23450)
}
