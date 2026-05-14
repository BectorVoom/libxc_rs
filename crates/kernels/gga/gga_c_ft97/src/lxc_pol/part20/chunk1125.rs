//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1125/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1125<F: Float>(t14012: F, t6154: F, t10052: F, t2569: F, t6940: F, t3864: F, t97299: F, t10002: F, t27986: F, t2568: F, t3972: F, t6187: F, t1449: F, t27929: F, t5996: F, t1173: F, t1403: F, t193: F, t2354: F, t2373: F, t2405: F, t2413: F, t24181: F, t24193: F, t28018: F, t3827: F, t6002: F, t6192: F, t6745: F, t9744: F, t98139: F) -> (F, F, F, F, F, F, F) {
    let t109606 = t6154 * t14012;
    let t109609 = t10052 * t6940 * t2569;
    let t109611 = t97299 * t3864;
    let t109617 = t10002 * t27986;
    let t109620 = t2568 * t6187 * t3972;
    let t109623 = t2568 * t1449 * t14012;
    let t109634 = t5996 * t27929 / 9.0;
    let t109636 = t1403 * t193 * t24181 * t1173 * t2373 - 2.0 * t109606 - 12.0 * t109609 + 8.0 * t109611 - 2.0 / 3.0 * t6745 * t24193 - 2.0 * t3827 * t6192 + 8.0 * t109617 + 8.0 * t109620 + 4.0 * t109623 - t6002 * t2354 * t28018 * t2413 / 18.0 - t6002 * t9744 * t28018 * t2405 / 27.0 - t109634 + t98139 / 81.0;
    (t109606, t109609, t109611, t109617, t109620, t109623, t109636)
}
