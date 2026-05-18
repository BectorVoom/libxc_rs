//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 984/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk984<F: Float>(t13238: F, t5559: F, t841: F, t34013: F, t977: F, t3073: F, t9767: F, t5552: F, t10687: F, t2554: F, t7064: F, t13200: F, t29439: F) -> (F, F, F, F, F, F) {
    let t42912 = t5559 * t13238 * t841;
    let t42914 = t34013 * t977;
    let t42916 = t9767 * t3073;
    let t42917 = t5552 * t13238;
    let t42931 = t7064 * t10687 * t2554;
    let t42933 = t29439 * t13200;
    (t42912, t42914, t42916, t42917, t42931, t42933)
}
