//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1326/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1326<F: Float>(t1882: F, t31787: F, t1501: F, t72745: F, t10703: F, t112785: F, t113716: F, t113722: F, t113749: F, t113773: F, t11593: F, t15195: F, t15369: F, t15460: F, t1901: F, t19324: F, t19455: F, t19563: F, t24898: F, t25044: F, t25253: F, t29083: F, t296: F, t31698: F, t31798: F, t4176: F, t44369: F, t446: F, t5330: F, t5413: F, t6334: F, t840: F) -> (F, F) {
    let t126269 = t1882 * t31787;
    let t126282 = t72745 * t1501;
    let t126286 = 2.0 / 3.0 * t446 * t840 * t25253 * t5330 + t113716 - t113722 + t113749 - 2.0 / 9.0 * t1901 * t44369 * t31798 + 2.0 / 9.0 * t1901 * t15195 * t29083 - 2.0 / 9.0 * t1901 * t10703 * t25044 * t5413 - 2.0 / 9.0 * t1901 * t10703 * t6334 * t19563 + 4.0 / 9.0 * t11593 * t10703 * t6334 * t19455 - 2.0 / 27.0 * t126269 - 2.0 / 9.0 * t1901 * t44369 * t31698 - 4.0 / 3.0 * t1901 * t15369 * t24898 * t19324 - 4.0 / 3.0 * t1901 * t15460 * t112785 * t4176 + t113773 - t446 * t296 * t126282 / 3.0;
    (t126282, t126286)
}
