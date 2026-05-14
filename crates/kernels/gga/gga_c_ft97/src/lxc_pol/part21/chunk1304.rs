//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1304/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1304<F: Float>(t30454: F, t8392: F, t1882: F, t30486: F, t106496: F, t107399: F, t119550: F, t119661: F, t119674: F, t12968: F, t144: F, t15772: F, t167: F, t17076: F, t1901: F, t2185: F, t2210: F, t26590: F, t26928: F, t30127: F, t30239: F, t30393: F, t3455: F, t379: F, t446: F, t4462: F, t49414: F, t50558: F, t569: F, t574: F, t5942: F, t5975: F, t616: F, t63180: F, t925: F) -> (F,) {
    let t120562 = t8392 * t30454;
    let t120568 = t1882 * t30486;
    let t120600 = -t446 * t569 * t5975 * t4462 / 9.0 - 4.0 / 3.0 * t1901 * t12968 * t5942 * t17076 - 4.0 / 3.0 * t1901 * t49414 * t30393 - 4.0 / 27.0 * t106496 + 4.0 / 27.0 * t120562 + 2.0 / 3.0 * t1901 * t50558 * t30127 * t379 + 2.0 / 9.0 * t120568 + 4.0 / 3.0 * t446 * t2185 * t167 * t119674 + 2.0 / 3.0 * t446 * t2185 * t616 * t30239 + 2.0 / 3.0 * t446 * t2185 * t167 * t119661 - 4.0 / 3.0 * t1901 * t63180 * t26928 + 2.0 / 3.0 * t446 * t574 * t26590 * t3455 + t1901 * t2210 * t5942 * t15772 / 9.0 + 2.0 / 9.0 * t1901 * t2210 * t107399 * t925 - t446 * t144 * t119550 / 3.0;
    (t120600,)
}
