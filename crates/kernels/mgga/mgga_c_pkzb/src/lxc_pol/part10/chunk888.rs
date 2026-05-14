//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 888/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk888<F: Float>(t1485: F, t178: F, t405: F, t404: F, t67: F, t931: F, t154: F, t2226: F, t385: F, t486: F) -> (F, F, F, F, F, F) {
    let t6428 = t178 * t1485 * t405;
    let t6430 = 0.63517063878621832551e-4 * t404 * t6428;
    let t6431 = t67 * t931;
    let t6433 = t154 * t6431 * t2226;
    let t6434 = t385 * t6433;
    let t6446 = t486 * t405;
    (t6428, t6430, t6431, t6433, t6434, t6446)
}
