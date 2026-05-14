//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1138/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1138<F: Float>(t1165: F, t1552: F, t1759: F, t3451: F, t864: F, t3379: F, t6271: F, t1487: F, t407: F, t1173: F, t1180: F, t1181: F, t1532: F, t18027: F, t18031: F, t18035: F, t18037: F, t18041: F, t18045: F, t18047: F, t301: F, t372: F, t5799: F) -> (F, F) {
    let t23429 = t3451 * t1165 * t1552 * t1759 * t864;
    let t23431 = t3379 * t6271;
    let t23445 = t407 * t1487;
    let t23450 = -0.34299214494455789578e-2 * t18027 - 0.34299214494455789578e-1 * t18031 - 0.17149607247227894789e-2 * t18035 + 0.32012600194825403606e-1 * t18037 - 0.85748036236139473944e-3 * t18041 + 0.17149607247227894789e-2 * t23429 + 0.68598428988911579156e-2 * t23431 - 0.85748036236139473944e-3 * t18045 + 0.16006300097412701803e-1 * t18047 + 0.68598428988911579156e-2 * t1173 * t1181 * t1532 * t5799 * t301 + 0.34299214494455789578e-2 * t1180 * t1181 * t1552 * t5799 * t372 - 0.17149607247227894789e-2 * t1180 * t1181 * t1532 * t23445;
    (t23445, t23450)
}
