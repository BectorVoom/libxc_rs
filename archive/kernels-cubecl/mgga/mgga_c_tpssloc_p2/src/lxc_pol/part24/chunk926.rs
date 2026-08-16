//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 926/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk926<F: Float>(t10523: F, t10524: F, t2932: F, t959: F, t10195: F, t2768: F, t123: F) -> (F, F) {
    let t10526 = t10523 * t10524 * t2932;
    let t10528 = F::cast_from(0.10389515463408878255e3_f64) * t959 * t10526;
    let t10529 = t2768 * t10195;
    let t10530 = t123 * t10529;
    (t10528, t10530)
}
