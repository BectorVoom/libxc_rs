//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1132/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1132<F: Float>(t1449: F, t9570: F, t28181: F, t8392: F, t28350: F, t28305: F, t24412: F, t737: F, t10007: F, t11593: F, t13863: F, t13893: F, t14075: F, t14175: F, t14176: F, t14187: F, t1901: F, t2413: F, t24789: F, t2579: F, t28145: F, t28344: F, t3875: F, t3880: F, t52054: F, t53942: F, t67996: F, t6921: F, t97259: F, t97261: F, t97267: F, t97537: F) -> (F,) {
    let t109890 = t1449 * t9570;
    let t109900 = 4.0 / 81.0 * t8392 * t28181;
    let t109902 = 2.0 / 27.0 * t8392 * t28350;
    let t109925 = 2.0 / 27.0 * t8392 * t28305;
    let t109926 = t737 * t24412;
    let t109930 = 2.0 / 27.0 * t1901 * t14187 * t28344 * t14075 + 10.0 / 81.0 * t1901 * t52054 * t109890 * t13863 - t1901 * t10007 * t6921 * t2413 / 9.0 - t109900 + t109902 + 8.0 / 3.0 * t1901 * t67996 * t6921 * t2579 - 2.0 / 9.0 * t1901 * t10007 * t97537 * t3875 - 4.0 / 9.0 * t1901 * t14175 * t97537 * t3880 - 4.0 / 3.0 * t1901 * t53942 * t28145 - 4.0 / 9.0 * t11593 * t24789 * t13893 - 2.0 / 27.0 * t97259 + 4.0 / 9.0 * t97261 + 4.0 / 9.0 * t97267 - t109925 - 4.0 / 9.0 * t1901 * t109926 * t14176;
    (t109930,)
}
