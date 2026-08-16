//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 895/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk895(t16779: f64, t16883: f64, t241: f64, t1000: f64, t16640: f64, t914: f64, t16632: f64, t2549: f64, t16648: f64, t13603: f64, t13607: f64, t13612: f64, t16654: f64, t16657: f64, t4054: f64, t5069: f64, t5076: f64, t999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16885 = t241 * (t16779 + t16883);
    let t16886 = t1000 * t16640;
    let t16887 = t914 * t16886;
    let t16890 = t2549 * t16632;
    let t16891 = t914 * t16890;
    let t16894 = t1000 * t16648;
    let t16895 = t914 * t16894;
    let t16900 = -t16654 - t16657 - t13603 / 3.0_f64 + t13607 / 3.0_f64 + t13612 / 6.0_f64 + t16885 + t999 * t16887 - t4054 * t5076 - 4.0_f64 / 3.0_f64 * t999 * t16891 + t999 * t16895 / 6.0_f64 + 2.0_f64 / 3.0_f64 * t4054 * t5069;
    (t16885, t16886, t16887, t16890, t16891, t16894, t16895, t16900)
}
