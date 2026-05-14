//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1331/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1331<F: Float>(t1882: F, t30376: F, t30500: F, t376: F, t89: F, t30359: F, t106807: F, t107470: F, t119646: F, t119785: F, t120117: F, t12968: F, t13140: F, t1391: F, t144: F, t15772: F, t16675: F, t167: F, t16971: F, t17365: F, t1901: F, t2185: F, t2205: F, t23571: F, t4454: F, t446: F, t4733: F, t47659: F, t47666: F, t569: F, t5975: F, t9432: F, t95521: F, t95842: F, t96160: F, t96167: F) -> (F,) {
    let t121476 = t1882 * t30376;
    let t121479 = t89 * t376 * t30500;
    let t121488 = t1882 * t30359;
    let t121505 = -2.0 * t446 * t9432 * t167 * t119785 - 4.0 / 3.0 * t1901 * t13140 * t95521 * t4733 - 4.0 / 27.0 * t96160 - 2.0 / 3.0 * t1901 * t12968 * t23571 * t16971 - 4.0 / 9.0 * t121476 - t121479 / 9.0 + 4.0 / 27.0 * t96167 - 8.0 / 27.0 * t47666 * t106807 * t16675 + 4.0 / 9.0 * t47659 * t95842 * t17365 + t121488 / 9.0 - 2.0 / 3.0 * t446 * t144 * t120117 + t107470 + 4.0 / 3.0 * t446 * t2185 * t167 * t119646 - t446 * t569 * t1391 * t15772 / 9.0 - 2.0 / 27.0 * t446 * t2205 * t5975 * t4454;
    (t121505,)
}
