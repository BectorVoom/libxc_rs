//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1508/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1508<F: Float>(t39597: F, t39604: F, t39606: F, t39608: F, t39615: F, t39635: F, t79935: F, t79942: F, t79946: F, t79952: F, t79953: F, t79954: F) -> F {
    let t80111 = -t79935 - t39597 + t39604 + t39606 + t39608 + t79942 + t39615 - t79946 - t39635 + t79952 + t79953 + t79954;
    t80111
}
