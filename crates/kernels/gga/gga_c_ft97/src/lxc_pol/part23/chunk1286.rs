//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1286/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1286<F: Float>(t31194: F, t8392: F, t10002: F, t30930: F, t110692: F, t111190: F, t11593: F, t122684: F, t123825: F, t124003: F, t124007: F, t13879: F, t13885: F, t14200: F, t17785: F, t18201: F, t18402: F, t18533: F, t1901: F, t242: F, t24737: F, t24793: F, t28187: F, t28368: F, t31110: F, t31163: F, t446: F, t53504: F, t53662: F, t53797: F, t53923: F, t67847: F, t97705: F, t9787: F) -> (F, F) {
    let t124836 = t8392 * t31194;
    let t124854 = t10002 * t30930;
    let t124876 = 2.0 / 27.0 * t1901 * t13879 * t31110 + 2.0 / 27.0 * t124836 + 2.0 / 9.0 * t1901 * t24793 * t18533 - 4.0 / 3.0 * t1901 * t67847 * t28368 - 4.0 / 3.0 * t1901 * t13885 * t24737 * t18201 + 4.0 / 9.0 * t53797 * t110692 * t17785 + 4.0 / 9.0 * t53797 * t97705 * t18402 + t111190 + 2.0 / 3.0 * t446 * t242 * t124854 + 2.0 / 9.0 * t1901 * t9787 * t31163 - 2.0 / 9.0 * t1901 * t53923 * t28187 - 4.0 / 9.0 * t1901 * t14200 * t123825 + 10.0 / 81.0 * t1901 * t53504 * t124003 - 8.0 / 27.0 * t11593 * t14200 * t124007 + 2.0 / 3.0 * t1901 * t53662 * t122684;
    (t124854, t124876)
}
