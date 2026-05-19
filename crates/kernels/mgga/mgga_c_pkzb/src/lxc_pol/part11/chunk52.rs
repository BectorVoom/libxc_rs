//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 52/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk52<F: Float>(t127: F, t83: F, t124: F, t99: F) -> (F, F, F) {
    let t128 = t83 * t127;
    let t130 = F::cast_from(0.19751673498613801407e-1_f64) * t99 * t124;
    let t131 = F::ln(F::new(2.0));
    let t132 = F::new(1.0) - t131;
    (t128, t130, t132)
}
