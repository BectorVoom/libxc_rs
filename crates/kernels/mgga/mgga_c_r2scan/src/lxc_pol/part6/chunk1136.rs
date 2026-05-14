//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1136/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1136<F: Float>(t1591: F, t6486: F, t1625: F, t6240: F, t1600: F, t6349: F, t128: F, t4145: F, t524: F, t540: F, t6235: F, t6425: F, t114: F, t6358: F, t252: F, t1569: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20609 = t1591 * t6486;
    let t20617 = t6240 * t1625;
    let t20619 = t1600 * t6349;
    let t20621 = t4145 * t128;
    let t20622 = t524 * t20621;
    let t20623 = t20622 * t540;
    let t20625 = t6425 * t6235;
    let t20642 = 1.0 / t6358 / t114;
    let t20643 = t20642 * t252;
    let t20646 = t1569 * t1569;
    (t20609, t20617, t20619, t20622, t20623, t20625, t20642, t20643, t20646)
}
