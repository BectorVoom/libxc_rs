//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1129/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1129<F: Float>(t17351: F, t17405: F, t17505: F, t20705: F, t25633: F, t25636: F, t25734: F, t25740: F, t25747: F, t25750: F, t25767: F, t30284: F, t30287: F, t30289: F, t30291: F, t30294: F, t30296: F, t30309: F, t30311: F) -> (F,) {
    let t30571 = -0.92617777777777777776e0 * t17405 - 0.48204333333333333333e1 * t20705 + 0.3529725e1 * t30289 + 0.6311625e0 * t30291 + 0.794188125e1 * t30294 - 0.473371875e0 * t30296 + t17505 - 0.16068111111111111111e1 * t17351 + 0.20659e1 * t25633 - 0.1549425e1 * t25636 + 0.104195e1 * t25734 - 0.516475e0 * t30284 + 0.1549425e1 * t30287 - 0.125034e1 * t25740 - 0.62517e0 * t25747 - 0.62517e0 * t25750 + 0.104195e1 * t25767 + 0.2366859375e0 * t30309 - 0.473371875e0 * t30311;
    (t30571,)
}
