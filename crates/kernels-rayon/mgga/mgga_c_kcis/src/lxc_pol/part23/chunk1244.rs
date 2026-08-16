//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1244/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1244(t52613: f64, t7908: f64, t8154: f64, t11825: f64, t491: f64, t1464: f64, t2038: f64, t4124: f64, t1386: f64, t16962: f64, t1307: f64, t1380: f64, t1943: f64, t94228: f64) -> (f64, f64, f64, f64) {
    let t98308 = t7908 * t52613 * t8154;
    let t98310 = t11825 * t491;
    let t98313 = t1464 * t98310 * t2038 * t4124;
    let t98315 = t16962 * t1386;
    let t98322 = t94228 * t1943 * t1380 * t1307;
    (t98308, t98313, t98315, t98322)
}
