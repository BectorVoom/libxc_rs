//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1262/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1262<F: Float>(t103821: F, t103823: F, t103827: F, t103832: F, t103835: F, t103837: F, t103840: F, t103849: F, t103855: F, t10951: F, t10961: F, t11450: F, t11490: F, t12046: F, t1825: F, t1901: F, t1912: F, t23265: F, t23327: F, t26372: F, t26373: F, t26390: F, t446: F, t452: F, t47548: F, t93676: F, t93677: F) -> (F,) {
    let t103859 = 2.0 / 3.0 * t1901 * t47548 * t23265 * t12046 - 22.0 / 27.0 * t103821 + 2.0 / 9.0 * t1901 * t103823 * t1912 + 8.0 * t1901 * t26372 * t103827 * t10951 - 4.0 / 27.0 * t103832 + t103835 - t103837 + t93676 - 8.0 / 27.0 * t93677 - t103840 + 2.0 / 3.0 * t446 * t452 * t1825 * t26390 - 4.0 * t1901 * t26372 * t26373 * t10961 + 2.0 * t1901 * t11490 * t103849 * t10951 + t103855 + t1901 * t23327 * t11450 / 9.0;
    (t103859,)
}
