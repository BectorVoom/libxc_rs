//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1012/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1012(t1724: f64, t2943: f64, t4667: f64, t932: f64, t13712: f64, t1035: f64, t3061: f64, t13714: f64, t45: f64, t4731: f64, t1666: f64, t2937: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13747 = t2943 * t1724;
    let t13750 = t932 * t4667;
    let t13781 = 0.18344444444444444444e-2_f64 * t13712;
    let t13790 = t3061 * t1035;
    let t13842 = 0.23744444444444444444e-1_f64 * t13714;
    let t13857 = t45 * t4731;
    let t13864 = t1666 * t2937;
    (t13747, t13750, t13781, t13790, t13842, t13857, t13864)
}
