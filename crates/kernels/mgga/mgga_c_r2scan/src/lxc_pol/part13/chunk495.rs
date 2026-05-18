//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 495/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk495<F: Float>(t2224: F, t529: F, t119: F, t1266: F, t122: F, t507: F, t1234: F, t506: F, t2168: F, t546: F) -> (F, F, F, F, F, F) {
    let t2225 = t529 * t2224;
    let t2228 = t1266 * t119;
    let t2231 = F::new(0.16463622957338778997e-1) * t2228 * t122 * t507;
    let t2232 = t506 * t1234;
    let t2233 = t529 * t2232;
    let t2236 = t546 * t2168;
    (t2225, t2228, t2231, t2232, t2233, t2236)
}
