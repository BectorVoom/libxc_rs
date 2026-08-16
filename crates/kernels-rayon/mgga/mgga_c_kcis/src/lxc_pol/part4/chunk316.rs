//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 316/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk316(t1154: f64, t1155: f64, t829: f64, t1083: f64, t304: f64, t1110: f64, t1115: f64, t1143: f64, t1152: f64, t1153: f64, t348: f64, t365: f64, t368: f64, t86: f64) -> (f64, f64, f64) {
    let t1157 = t1154 * t1155 * t829;
    let t1160 = t304 * t1083;
    let t1164 = 0.619125e-2_f64 * t1143 * t348 + 0.9286875e-2_f64 * t365 * t1110 - 0.619125e-2_f64 * t365 * t1115 - t1152 - 0.26531111111111111111e-1_f64 * t1153 * t1157 - 0.39796666666666666666e-1_f64 * t86 * t368 * t1160;
    (t1157, t1160, t1164)
}
