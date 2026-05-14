//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1193/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1193<F: Float>(t668: F, t6940: F, t31130: F, t8392: F, t31014: F, t684: F, t109448: F, t3886: F, t10007: F, t10085: F, t109900: F, t109902: F, t1160: F, t14163: F, t14175: F, t14200: F, t1901: F, t28141: F, t28145: F, t28340: F, t28345: F, t31193: F, t31234: F, t31244: F, t31306: F, t3875: F, t3880: F, t42339: F, t51901: F, t65408: F, t67796: F, t68007: F, t9707: F) -> (F, F, F) {
    let t121988 = t6940 * t668;
    let t122001 = t8392 * t31130;
    let t122003 = t31014 * t684;
    let t122007 = t109448 * t3886;
    let t122031 = 4.0 / 27.0 * t1901 * t68007 * t28345 - 2.0 / 9.0 * t1901 * t10007 * t121988 * t3875 - 4.0 / 9.0 * t1901 * t14175 * t121988 * t3880 + 2.0 / 9.0 * t1901 * t42339 * t31244 * t684 + 2.0 / 27.0 * t122001 - t109900 + t109902 - 4.0 / 9.0 * t1901 * t14163 * t122003 + 4.0 / 27.0 * t1901 * t14200 * t122007 - t1901 * t10007 * t31234 * t684 / 9.0 - 4.0 / 9.0 * t1901 * t67796 * t28340 - 4.0 / 9.0 * t1901 * t51901 * t31306 - 4.0 * t1901 * t9707 * t1160 * t28141 - 4.0 / 3.0 * t1901 * t65408 * t28145 - 2.0 / 9.0 * t1901 * t10085 * t31193;
    (t122003, t122007, t122031)
}
