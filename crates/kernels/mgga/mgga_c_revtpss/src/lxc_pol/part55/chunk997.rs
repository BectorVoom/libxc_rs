//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 997/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk997<F: Float>(t1113: F, t2411: F, t2453: F, t555: F, t25304: F, t1444: F, t543: F, t1419: F, t7063: F, t25081: F, t7234: F, t606: F, t68: F, t198: F, t206: F, t7427: F) -> (F, F, F, F, F, F, F, F) {
    let t94245 = t2411 * t1113;
    let t94382 = t2453 * t555;
    let t94390 = t25304 * t555;
    let t94396 = t543 * t1444;
    let t94801 = t7063 * t1419;
    let t95088 = t7234 * t25081;
    let t95334 = t606 * t68;
    let t95511 = t198 * t206 * t7427;
    (t94245, t94382, t94390, t94396, t94801, t95088, t95334, t95511)
}
