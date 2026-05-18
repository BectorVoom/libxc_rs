//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 171/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk171<F: Float>(t114: F, t534: F, t477: F, t479: F, t483: F, t488: F) -> (F, F) {
    let t535 = t114 * t534;
    let t540 = -F::new(0.86308333333333333334e0) * t477 - F::new(0.301925e0) * t479 - F::new(0.5501625e-1) * t483 - F::new(0.82785e-1) * t488;
    (t535, t540)
}
