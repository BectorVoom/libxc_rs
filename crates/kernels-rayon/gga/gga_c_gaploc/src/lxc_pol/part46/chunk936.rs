//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 936/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk936(t13086: f64, t64: f64, t10657: f64, t871: f64, t2919: f64, t3113: f64, t40612: f64, t40614: f64, t40620: f64, t40630: f64, t40632: f64, t40634: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43071 = 4.0_f64 / 3.0_f64 * t13086 * t64;
    let t43072 = t10657 * t871;
    let t43073 = t2919 * t3113;
    let t43075 = 7.0_f64 / 512.0_f64 * t40612;
    let t43076 = 63.0_f64 / 16384.0_f64 * t40614;
    let t43077 = 63.0_f64 / 1048576.0_f64 * t40620;
    let t43078 = 21.0_f64 / 1048576.0_f64 * t40630;
    let t43079 = 21.0_f64 / 16384.0_f64 * t40632;
    let t43080 = 7.0_f64 / 1536.0_f64 * t40634;
    (t43071, t43072, t43073, t43075, t43076, t43077, t43078, t43079, t43080)
}
