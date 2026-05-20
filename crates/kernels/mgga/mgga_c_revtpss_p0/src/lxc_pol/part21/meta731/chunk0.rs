//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2576/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2576<F: Float>(t675: F, t9898: F, t268: F, t4101: F, t543: F, t14192: F, t555: F, t786: F, t9994: F, t10023: F, t4003: F, t10115: F, t1441: F) -> (F, F, F, F, F) {
    let t47366 = t675 * t9898;
    let t47369 = t4101 * t268 * t47366 * t543;
    let t47371 = t14192 * t555;
    let t47372 = t786 * t47371;
    let t47375 = t47372 * t268 * t47366 * t9994;
    let t47379 = t10023 * t268 * t47366 * t4003;
    let t47381 = t10115 * t1441;
    (t47369, t47371, t47375, t47379, t47381)
}
