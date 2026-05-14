//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1334/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1334<F: Float>(t1882: F, t30380: F, t30528: F, t106981: F, t107311: F, t107589: F, t107603: F, t107614: F, t107621: F, t119492: F, t120110: F, t12277: F, t12968: F, t144: F, t16666: F, t16671: F, t16977: F, t17123: F, t17496: F, t1901: F, t23571: F, t27334: F, t27335: F, t446: F, t47659: F, t574: F, t6639: F, t95837: F, t96244: F) -> (F,) {
    let t121583 = t1882 * t30380;
    let t121602 = t1882 * t30528;
    let t121604 = -t446 * t144 * t120110 / 3.0 - 2.0 * t1901 * t27334 * t27335 * t16977 - 4.0 / 3.0 * t1901 * t12968 * t23571 * t17123 - t107589 - 2.0 / 9.0 * t121583 - t446 * t144 * t119492 / 3.0 - t107603 + t96244 + 8.0 / 81.0 * t107614 + 4.0 / 9.0 * t47659 * t106981 * t16666 + 4.0 / 9.0 * t47659 * t95837 * t17496 + 4.0 / 9.0 * t47659 * t107311 * t16671 + t107621 + 2.0 / 3.0 * t446 * t574 * t12277 * t6639 - 4.0 / 9.0 * t121602;
    (t121604,)
}
