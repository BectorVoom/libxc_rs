//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 227/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk227<F: Float>(t138: F, t125: F, t126: F, t67: F, t117: F, t120: F) -> (F, F, F, F, F) {
    let t681 = t138 * t138;
    let t682 = F::cast_from(1.0_f64) / t681;
    let t683 = t125 * t682;
    let t685 = F::cast_from(1.0_f64) / t126 * t67;
    let t686 = t117 * t120;
    (t681, t682, t683, t685, t686)
}
