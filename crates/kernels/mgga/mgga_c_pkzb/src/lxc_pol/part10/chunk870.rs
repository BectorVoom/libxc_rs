//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 870/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk870<F: Float>(t197: F, t5952: F, t2030: F, t287: F, t5728: F, t655: F, t54: F, t779: F) -> (F, F, F, F, F) {
    let t5953 = t5952 * t197;
    let t5955 = t2030 * t287;
    let t5956 = t5728 * t5955;
    let t5965 = t2030 * t655;
    let t5974 = t54 * t779;
    (t5953, t5955, t5956, t5965, t5974)
}
