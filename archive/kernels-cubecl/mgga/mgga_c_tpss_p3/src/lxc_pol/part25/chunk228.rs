//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 228/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk228<F: Float>(t57: F, t581: F, t745: F, t744: F, zeta_threshold: F) -> F {
    let t155 = t57 <= zeta_threshold;
    let t748 = piecewise3::<F>(t155, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t745 * t581);
    let t750 = t744 / F::cast_from(2.0_f64) + t748 / F::cast_from(2.0_f64);
    t750
}
