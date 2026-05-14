//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1152/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1152<F: Float>(t1882: F, t28218: F, t28130: F, t8392: F, t10051: F, t1424: F, t28133: F, t28286: F, t10007: F, t10085: F, t108004: F, t13858: F, t13863: F, t13885: F, t14053: F, t14058: F, t14127: F, t14145: F, t14163: F, t14182: F, t1901: F, t2409: F, t24429: F, t24526: F, t24569: F, t24668: F, t2486: F, t2599: F, t28123: F, t28124: F, t28344: F, t3859: F, t3880: F, t3893: F, t446: F, t53927: F, t6148: F, t6161: F, t729: F) -> (F,) {
    let t110559 = 4.0 / 9.0 * t1882 * t28218;
    let t110575 = 4.0 / 9.0 * t8392 * t28130;
    let t110576 = t10051 * t1424;
    let t110582 = 2.0 / 27.0 * t8392 * t28133;
    let t110588 = 2.0 / 9.0 * t1882 * t28286;
    let t110609 = -t110559 + 2.0 / 9.0 * t1901 * t10085 * t28124 - 4.0 / 9.0 * t1901 * t14163 * t108004 - 2.0 / 9.0 * t1901 * t10007 * t24526 * t3880 - 2.0 / 9.0 * t1901 * t2599 * t28123 * t2409 + t110575 + 2.0 * t1901 * t14127 * t110576 * t14145 - t110582 + 2.0 / 3.0 * t446 * t729 * t24429 * t3859 - t110588 - 4.0 / 27.0 * t1901 * t2486 * t6148 * t3893 - 4.0 / 3.0 * t1901 * t13885 * t6161 * t14053 - 4.0 / 3.0 * t1901 * t14127 * t24668 * t14058 + 2.0 / 3.0 * t1901 * t53927 * t24569 * t13858 + 2.0 / 3.0 * t1901 * t14182 * t28344 * t13863;
    (t110609,)
}
