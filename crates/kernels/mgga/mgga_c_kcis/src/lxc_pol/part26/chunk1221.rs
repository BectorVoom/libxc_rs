//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1221/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1221<F: Float>(t1938: F, t1943: F, t1307: F, t94228: F, t102431: F, t102441: F, t102444: F, t102447: F, t102450: F, t102454: F, t102457: F, t103206: F, t103331: F, t27369: F, t5426: F, t8148: F, t94227: F, t94626: F, t98315: F, t98733: F) -> (F,) {
    let t103534 = t1938 * t1943;
    let t103536 = t94228 * t103534 * t1307;
    let t103551 = 0.61782407407407407408e-3 * t94626 * t98315 * t5426 * t103331 - 0.22109259259259259259e-2 * t102431 - 0.46336805555555555557e-3 * t94626 * t103536 - 0.6183646701388888889e-4 * t94227 * t103536 + 0.18550940104166666667e-3 * t98733 * t8148 + 0.18424382716049382715e-2 * t102441 + 0.33163888888888888888e-2 * t102444 - 0.66327777777777777776e-2 * t102447 + 0.16581944444444444444e-2 * t102450 + 0.16581944444444444444e-2 * t102454 + 0.27636574074074074073e-2 * t102457 + 0.18550940104166666667e-3 * t27369 * t103206;
    (t103551,)
}
