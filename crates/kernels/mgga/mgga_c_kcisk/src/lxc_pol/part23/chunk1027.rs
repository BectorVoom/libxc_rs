//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1027/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1027<F: Float>(t20066: F, t20095: F, t20134: F, t20164: F, t20193: F, t20214: F, t20241: F, t20614: F, t1328: F, t1341: F, t442: F, t1340: F, t5600: F, t3924: F, t6217: F, t1327: F) -> (F, F, F, F, F, F) {
    let t20617 = t20066 + t20095 + t20134 + t20164 + t20193 + t20214 + t20241 + t20614;
    let t20618 = t20617 * t1328;
    let t20621 = t1341 * t442;
    let t20622 = t1340 * t20621;
    let t20623 = t5600 * t20622;
    let t20625 = t6217 * t3924;
    let t20626 = t20625 * t1327;
    (t20617, t20618, t20621, t20623, t20625, t20626)
}
