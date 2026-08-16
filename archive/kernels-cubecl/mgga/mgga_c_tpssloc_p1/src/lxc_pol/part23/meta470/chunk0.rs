//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1398/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1398<F: Float>(t77119: F, t77122: F, t77124: F, t77127: F, t77130: F, t77133: F, t77135: F, t77138: F, t77143: F, t77145: F, t77148: F, t77150: F, t77153: F) -> F {
    let t77920 = t77119 - t77122 - t77124 + t77127 - t77130 - t77133 + t77135 + t77138 + t77143 - t77145 - t77148 - t77150 - t77153;
    t77920
}
