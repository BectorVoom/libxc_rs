//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 310/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk310<F: Float>(t27: F, t983: F, t23: F, t28: F, t7: F, t974: F, t980: F) -> (F, F) {
    let t984 = t27 * t983;
    let t987 = 5.0 / 3.0 * t7 * t974 - 8.0 / 3.0 * t980 * t28 + 5.0 / 3.0 * t23 * t984;
    (t984, t987)
}
