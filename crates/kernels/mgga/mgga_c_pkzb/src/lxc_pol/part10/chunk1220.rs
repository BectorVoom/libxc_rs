//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1220/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1220<F: Float>(t6491: F, t8360: F, t1238: F, t6400: F, t1167: F, t179: F, t19150: F, t404: F, t2411: F, t2888: F, t2099: F, t3235: F, t8419: F, t8410: F, t8414: F, t23213: F, t3206: F, t8255: F) -> (F, F, F, F, F, F, F, F) {
    let t23264 = t8360 * t6491;
    let t23266 = t1238 * t6400;
    let t23272 = t404 * t179 * t19150 * t1167;
    let t23278 = t2888 * t2411;
    let t23286 = t3235 * t2099 * t8419;
    let t23296 = t3235 * t2099 * t8410;
    let t23299 = t3235 * t2099 * t8414;
    let t23311 = t3206 * t23213 * t8255;
    (t23264, t23266, t23272, t23278, t23286, t23296, t23299, t23311)
}
