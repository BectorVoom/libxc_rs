//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 700/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk700<F: Float>(t1101: F, t1165: F, t604: F, t7493: F, t1106: F, t1181: F, t7426: F, t2085: F, t372: F, t4262: F, t2030: F, t182: F, t592: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7495 = t1165 * t604 * t1101;
    let t7496 = t7493 * t7495;
    let t7497 = F::cast_from(0.15724046144802076034e-2_f64) * t7496;
    let t7499 = t1181 * t604 * t1106;
    let t7500 = t7426 * t7499;
    let t7502 = t2085 * t372;
    let t7503 = t4262 * t7502;
    let t7504 = t2030 * t7503;
    let t7506 = t182 * t592;
    (t7495, t7496, t7497, t7499, t7500, t7502, t7503, t7504, t7506)
}
