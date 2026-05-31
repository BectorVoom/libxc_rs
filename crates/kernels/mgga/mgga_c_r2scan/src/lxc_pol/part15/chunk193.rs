//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 193/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk193<F: Float>(t495: F, t551: F, t552: F, t239: F, t378: F, t5: F, t152: F, t153: F, t158: F) -> (F, F, F, F) {
    let t576 = t551 * t552 * t495;
    let t581 = F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t5 * t378 * t239;
    let t583 = F::cast_from(1.0_f64) / t153 / t152;
    let t584 = t583 * t158;
    (t576, t581, t583, t584)
}
