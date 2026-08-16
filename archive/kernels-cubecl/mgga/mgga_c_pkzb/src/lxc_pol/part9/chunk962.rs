//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 962/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk962<F: Float>(t1306: F, t2149: F, t2153: F, t2997: F, t7225: F, t7230: F, t7265: F, t7268: F, t7271: F, t7274: F, t7277: F, t7281: F, t7284: F, t7288: F, t7524: F, t7526: F, t7530: F, t7534: F, t7538: F, t7540: F, t7543: F, t7548: F) -> F {
    let t7549 = -t1306 * t2149 * t2997 + F::cast_from(2.0_f64) * t1306 * t2153 * t7543 - t7225 - t7230 - t7265 + t7268 - t7271 - t7274 - t7277 + t7281 + t7284 + t7288 + t7524 + t7526 + t7530 + t7534 - t7538 - t7540 - t7548;
    t7549
}
