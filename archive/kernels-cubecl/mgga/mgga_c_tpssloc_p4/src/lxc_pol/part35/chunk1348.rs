//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1348/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1348<F: Float>(t55921: F, t7245: F, t12571: F, t27331: F, t2240: F, t29473: F, t33: F, t111: F, t29485: F, t112: F, t29865: F, t1851: F, t8119: F) -> (F, F, F, F, F, F) {
    let t104953 = t55921 * t7245;
    let t104958 = t12571 * t27331;
    let t104968 = t2240 * t33 * t29473;
    let t104990 = t29485 * t111;
    let t105105 = t29865 * t112;
    let t105131 = t1851 * t8119;
    (t104953, t104958, t104968, t104990, t105105, t105131)
}
