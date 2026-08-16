//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 748/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk748(t112: f64, t7945: f64, t19299: f64, t33: f64, t22505: f64, t22510: f64, t5392: f64, t5398: f64, t6500: f64, t67: f64, t1864: f64, t7441: f64, t7445: f64) -> (f64, f64, f64, f64, f64) {
    let t27254 = t7945 * t112;
    let t27937 = t19299 * t33;
    let t27948 = 5.0_f64 / 18.0_f64 * t22505 * t5392 + 5.0_f64 / 6.0_f64 * t6500 * t5398 - t22510;
    let t27949 = t27948 * t67;
    let t27950 = t27949 * t1864;
    let t27953 = t7441 * t7445;
    (t27254, t27937, t27948, t27950, t27953)
}
