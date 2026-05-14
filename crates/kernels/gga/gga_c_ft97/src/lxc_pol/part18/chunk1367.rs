//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1367/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1367<F: Float>(t582: F, t6685: F, t157: F, t40266: F, t27257: F, t8392: F, t27015: F, t50249: F, t604: F, t6615: F, t1378: F, t9224: F, t11593: F, t12725: F, t12968: F, t13003: F, t13070: F, t13221: F, t1901: F, t2190: F, t2213: F, t26863: F, t26883: F, t27207: F, t27211: F, t27252: F, t379: F, t40945: F, t47659: F, t51170: F, t5855: F, t9144: F, t95625: F, t95632: F, t95634: F, t95636: F) -> (F,) {
    let t106551 = t582 * t6685;
    let t106555 = t40266 * t157;
    let t106561 = 2.0 / 27.0 * t8392 * t27257;
    let t106565 = t50249 * t27015;
    let t106573 = t604 * t6615;
    let t106588 = t9224 * t1378;
    let t106595 = 2.0 / 9.0 * t1901 * t106551 * t2213 + 8.0 * t1901 * t106555 * t5855 * t13070 + t106561 - 4.0 / 9.0 * t1901 * t51170 * t27252 + 4.0 / 9.0 * t47659 * t106565 * t13221 - 2.0 / 9.0 * t95625 + 16.0 / 27.0 * t95632 + 2.0 / 3.0 * t95634 + 2.0 / 9.0 * t95636 - 4.0 / 3.0 * t1901 * t12968 * t106573 * t2190 - 2.0 / 9.0 * t1901 * t9144 * t26883 * t379 - 2.0 / 9.0 * t1901 * t40945 * t27207 - 4.0 / 9.0 * t1901 * t51170 * t27211 - 10.0 / 81.0 * t1901 * t106588 * t12725 - 8.0 / 27.0 * t11593 * t26863 * t13003;
    (t106595,)
}
