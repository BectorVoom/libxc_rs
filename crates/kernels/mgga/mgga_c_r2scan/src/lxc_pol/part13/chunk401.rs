//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 401/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk401<F: Float>(t1632: F, t495: F, t551: F, t574: F, t1554: F, t552: F, t1398: F, t239: F, t5: F, t378: F, t753: F, t621: F) -> (F, F, F, F, F, F) {
    let t1634 = t551 * t1632 * t495;
    let t1635 = t574 * t1634;
    let t1638 = t551 * t552 * t1554;
    let t1643 = 20.0 / 9.0 * t5 * t1398 * t239;
    let t1645 = t5 * t378 * t753;
    let t1647 = t621 * t621;
    (t1634, t1635, t1638, t1643, t1645, t1647)
}
