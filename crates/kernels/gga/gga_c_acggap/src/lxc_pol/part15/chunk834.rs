//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 834/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk834<F: Float>(t142: F, t9704: F, t2060: F, t2297: F, t513: F, t4262: F, t2030: F, t1755: F, t7822: F, t1761: F, t1859: F, t604: F) -> (F, F, F, F, F, F, F, F) {
    let t9705 = t142 * t9704;
    let t9706 = t2060 * t9705;
    let t9711 = t2297 * t513;
    let t9712 = t4262 * t9711;
    let t9713 = t2030 * t9712;
    let t9715 = t7822 * t1755;
    let t9717 = t7822 * t1761;
    let t9719 = t604 * t1859;
    (t9705, t9706, t9711, t9712, t9713, t9715, t9717, t9719)
}
