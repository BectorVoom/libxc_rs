//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1918/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1918<F: Float>(t1774: F, t7627: F, t7637: F, t1294: F, t8190: F, t7652: F, t1203: F, t8201: F, t1214: F, t8208: F, t2142: F, t5219: F) -> (F, F, F, F, F, F, F, F) {
    let t29278 = t7627 * t1774;
    let t29279 = t7637 * t29278;
    let t29282 = t8190 * t1294;
    let t29283 = t7652 * t29282;
    let t29287 = t7637 * t8201 * t1203;
    let t29292 = t8208 * t1214;
    let t29293 = t7652 * t29292;
    let t29296 = t8208 * t1203;
    let t29297 = t7652 * t29296;
    let t29300 = t8190 * t1203;
    let t29301 = t7637 * t29300;
    let t29304 = t5219 * t2142;
    (t29278, t29279, t29283, t29287, t29293, t29297, t29301, t29304)
}
