//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1410/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1410<F: Float>(t123: F, t2734: F, t33959: F, t109494: F, t33816: F, t9536: F, t1163: F, t22009: F, t32464: F, t2059: F, t32465: F, t4513: F, t14612: F, t442: F, t4348: F, t12951: F, t1597: F) -> (F, F, F, F, F, F) {
    let t115118 = t2734 * t33959 * t123;
    let t115137 = 0.11574074074074074074e-2 * t9536 * t109494 * t33816;
    let t115139 = t32464 * t22009 * t1163;
    let t115144 = t32464 * t32465 * t2059 * t4513;
    let t115147 = t14612 * t442;
    let t115150 = t32464 * t115147 * t2059 * t4348;
    let t115157 = t1597 * t12951;
    (t115118, t115137, t115139, t115144, t115150, t115157)
}
