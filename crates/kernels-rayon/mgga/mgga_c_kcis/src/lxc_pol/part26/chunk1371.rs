//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1371/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1371(t1938: f64, t1943: f64, t1307: f64, t94228: f64, t102431: f64, t102441: f64, t102444: f64, t102447: f64, t102450: f64, t102454: f64, t102457: f64, t103206: f64, t103331: f64, t27369: f64, t5426: f64, t8148: f64, t94227: f64, t94626: f64, t98315: f64, t98733: f64) -> f64 {
    let t103534 = t1938 * t1943;
    let t103536 = t94228 * t103534 * t1307;
    let t103551 = 0.61782407407407407408e-3_f64 * t94626 * t98315 * t5426 * t103331 - 0.22109259259259259259e-2_f64 * t102431 - 0.46336805555555555557e-3_f64 * t94626 * t103536 - 0.6183646701388888889e-4_f64 * t94227 * t103536 + 0.18550940104166666667e-3_f64 * t98733 * t8148 + 0.18424382716049382715e-2_f64 * t102441 + 0.33163888888888888888e-2_f64 * t102444 - 0.66327777777777777776e-2_f64 * t102447 + 0.16581944444444444444e-2_f64 * t102450 + 0.16581944444444444444e-2_f64 * t102454 + 0.27636574074074074073e-2_f64 * t102457 + 0.18550940104166666667e-3_f64 * t27369 * t103206;
    t103551
}
