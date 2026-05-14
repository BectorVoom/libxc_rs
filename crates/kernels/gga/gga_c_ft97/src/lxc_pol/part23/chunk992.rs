//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 992/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk992<F: Float>(t231: F, t5025: F, t1091: F, t1096: F, t24278: F, t200: F, t213: F, t6757: F, t27618: F, t17965: F, t17959: F, t232: F, t203: F, t5005: F, t1411: F, t6762: F, t6777: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t30590 = t231 * t5025;
    let t30594 = t1096 * t1091;
    let t30595 = t24278 * t30594;
    let t30598 = t200 * t213;
    let t30599 = t6757 * t30598;
    let t30600 = t27618 * t30599;
    let t30603 = t6757 * t17965;
    let t30607 = t6757 * t17959;
    let t30608 = t232 * t30607;
    let t30612 = t203 * t5005;
    let t30613 = t30612 * t1411;
    let t30615 = t6762 * t6777;
    (t30590, t30594, t30595, t30598, t30599, t30600, t30603, t30607, t30608, t30612, t30613, t30615)
}
