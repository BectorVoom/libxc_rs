//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1192/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1192<F: Float>(t10007: F, t109890: F, t109926: F, t111330: F, t11593: F, t14175: F, t14187: F, t17785: F, t18433: F, t18446: F, t18471: F, t18497: F, t18514: F, t18519: F, t18532: F, t18707: F, t18724: F, t1901: F, t24569: F, t28208: F, t28344: F, t28387: F, t31302: F, t42575: F, t5165: F, t5170: F, t52054: F, t67796: F, t68559: F, t97537: F, t97777: F) -> (F,) {
    let t121983 = 10.0 / 81.0 * t1901 * t52054 * t109890 * t18514 - 8.0 / 27.0 * t11593 * t14187 * t28344 * t18497 - 2.0 / 9.0 * t1901 * t42575 * t31302 - 2.0 / 9.0 * t1901 * t10007 * t97537 * t5165 - 2.0 / 9.0 * t1901 * t10007 * t24569 * t18724 - 4.0 / 9.0 * t1901 * t14175 * t97537 * t5170 - 4.0 / 9.0 * t1901 * t14175 * t24569 * t18532 + 8.0 / 9.0 * t11593 * t14175 * t24569 * t18519 - 2.0 / 9.0 * t1901 * t97777 * t18446 - 4.0 / 9.0 * t1901 * t109926 * t18433 - t1901 * t10007 * t24569 * t18471 / 9.0 - 2.0 / 9.0 * t1901 * t14175 * t24569 * t18707 - 4.0 / 9.0 * t1901 * t111330 * t17785 - 4.0 / 9.0 * t1901 * t67796 * t28208 - 4.0 / 9.0 * t1901 * t68559 * t28387;
    (t121983,)
}
