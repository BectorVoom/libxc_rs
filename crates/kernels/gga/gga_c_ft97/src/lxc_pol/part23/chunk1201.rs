//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1201/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1201<F: Float>(t1882: F, t31315: F, t10007: F, t110489: F, t110496: F, t111016: F, t111085: F, t111356: F, t11593: F, t14127: F, t14187: F, t17785: F, t17790: F, t17794: F, t18402: F, t18472: F, t18486: F, t18524: F, t18712: F, t1901: F, t24569: F, t24668: F, t24789: F, t28344: F, t31063: F, t31220: F, t3842: F, t42334: F, t446: F, t5181: F, t53797: F, t53927: F, t54032: F, t6061: F, t684: F, t729: F, t98123: F) -> (F,) {
    let t122399 = t1882 * t31315;
    let t122408 = 2.0 / 27.0 * t1901 * t14187 * t28344 * t18712 + 2.0 / 3.0 * t1901 * t53927 * t31063 * t684 + 2.0 / 9.0 * t1901 * t42334 * t31220 * t684 + 4.0 / 3.0 * t53797 * t111356 * t17785 + 4.0 / 9.0 * t53797 * t98123 * t18402 + 8.0 / 9.0 * t53797 * t111085 * t17790 - 8.0 / 27.0 * t54032 * t111085 * t17794 - t446 * t729 * t5181 * t6061 / 3.0 + 4.0 / 9.0 * t11593 * t10007 * t24569 * t18524 - 2.0 / 3.0 * t1901 * t14127 * t24668 * t18486 + t122399 / 9.0 - 4.0 / 3.0 * t1901 * t14127 * t111016 * t3842 + t1901 * t24789 * t18472 / 9.0 - t110489 - t110496;
    (t122408,)
}
