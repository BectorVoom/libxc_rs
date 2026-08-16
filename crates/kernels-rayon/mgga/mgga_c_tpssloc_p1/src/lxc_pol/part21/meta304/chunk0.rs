//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1649/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1649(t11135: f64, t11203: f64, t1128: f64, t3324: f64, t1124: f64, t3356: f64, t3355: f64, t432: f64, t427: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11369 = 0.93932222222222222223e0_f64 * t11135;
    let t11372 = 0.36793333333333333333e0_f64 * t11203;
    let t11410 = t3324 * t1128;
    let t11415 = t1124 * t3356;
    let t11419 = 1.0_f64 / t3355 / t432;
    let t11420 = t427 * t11419;
    (t11369, t11372, t11410, t11415, t11419, t11420)
}
