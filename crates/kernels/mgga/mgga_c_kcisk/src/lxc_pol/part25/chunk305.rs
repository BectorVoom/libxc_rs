//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 305/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk305<F: Float>(t1634: F, t1638: F, t1649: F, t1648: F, t1815: F, t574: F) -> (F, F, F) {
    let t1819 = 0.41275e-2 * t1634;
    let t1821 = 0.1982e-1 * t1649 - t1819 - 0.41275e-2 * t1638;
    let t1824 = t1815 * t1648 / 4.0 + t574 * t1821 / 2.0;
    (t1819, t1821, t1824)
}
