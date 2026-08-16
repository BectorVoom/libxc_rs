//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1220/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1220(t14656: f64, t795: f64, t3270: f64, t3269: f64, t23987: f64, t3263: f64, t3275: f64, t38259: f64, t38261: f64, t38265: f64, t38268: f64, t38270: f64, t40619: f64, t40623: f64, t40626: f64, t40628: f64, t40634: f64, t40638: f64, t40642: f64, t40647: f64) -> (f64, f64, f64) {
    let t40648 = t14656 * t795;
    let t40649 = t3270 * t40648;
    let t40651 = t3269 * t40649 / 2.0_f64;
    let t40652 = t23987 * t795;
    let t40654 = t3275 * t3263 * t40652;
    let t40655 = t40619 - t40623 - t40626 - t40628 - t40634 + t40638 - 0.30487649791575028314e-3_f64 * t38259 + 0.30487649791575028314e-3_f64 * t38261 - t38265 - t38268 - t38270 + 0.30487649791575028314e-3_f64 * t40642 - t40647 + t40651 - t40654;
    (t40651, t40654, t40655)
}
