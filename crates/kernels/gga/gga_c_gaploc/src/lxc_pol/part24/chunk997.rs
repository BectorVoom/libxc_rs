//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 997/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk997<F: Float>(t20671: F, t22629: F, t28309: F, t10007: F, t1865: F, t825: F, t9438: F, t10012: F, t2684: F, t22623: F, t7427: F, t10009: F, t2013: F, t10004: F, t5676: F, t1645: F, t7124: F) -> (F, F, F, F, F, F, F) {
    let t28312 = 0.34082600847929555268e0 * t28309 * t20671 * t22629;
    let t28357 = t825 * t9438 * t10007 * t1865;
    let t28361 = t2684 * t9438 * t10012 * t1865;
    let t28366 = t7427 * t9438 * t22623 * t1865;
    let t28378 = t2013 * t10009;
    let t28381 = 0.11916829983950142223e0 * t5676 * t10004;
    let t28387 = t1645 * t7124;
    (t28312, t28357, t28361, t28366, t28378, t28381, t28387)
}
