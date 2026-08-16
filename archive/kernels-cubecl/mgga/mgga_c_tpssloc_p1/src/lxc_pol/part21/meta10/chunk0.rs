//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 74/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk74<F: Float>(t123: F, t126: F, t129: F, t136: F) -> (F, F, F) {
    let t164 = F::cast_from(0.705945e1_f64) * t126 + F::cast_from(0.1549425e1_f64) * t123 + F::cast_from(0.420775e0_f64) * t129 + F::cast_from(0.1562925e0_f64) * t136;
    let t167 = F::cast_from(1.0_f64) + F::cast_from(0.32163958997385070134e2_f64) / t164;
    let t168 = F::ln(t167);
    (t164, t167, t168)
}
