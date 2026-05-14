//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 475/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk475<F: Float>(t1663: F, t1664: F, t390: F, t124: F, t4: F, t615: F) -> (F, F, F, F) {
    let t1665 = t1663 * t1664;
    let t1667 = 0.57278650314509912396e0 * t390 * t1665;
    let t1668 = t4 * t124;
    let t1669 = t615 * t1668;
    (t1665, t1667, t1668, t1669)
}
