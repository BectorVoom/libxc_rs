//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1381/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1381<F: Float>(t1874: F, t96238: F, t27863: F, t6535: F, t120045: F, t120047: F, t120049: F, t120051: F, t120053: F, t120055: F, t120057: F, t120063: F, t31055: F, t31057: F, t31060: F) -> F {
    let t123155 = t96238 * t1874;
    let t123164 = t27863 * t6535;
    let t123166 = -F::cast_from(2.0_f64) * t123155 - F::cast_from(2.0_f64) * t120045 - F::cast_from(2.0_f64) * t120047 - F::cast_from(2.0_f64) * t120049 - F::cast_from(2.0_f64) * t120051 - F::cast_from(2.0_f64) * t120053 - F::cast_from(2.0_f64) * t120055 - F::cast_from(2.0_f64) * t120057 - t31055 - t31057 - t31060 - F::cast_from(2.0_f64) * t123164 - t120063;
    t123166
}
