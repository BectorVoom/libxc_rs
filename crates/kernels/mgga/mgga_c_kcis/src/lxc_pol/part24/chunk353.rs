//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 353/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk353<F: Float>(t413: F, t1253: F, t1646: F, t1252: F, t1844: F) -> (F, F, F) {
    let t418 = F::cast_from(0.0_f64) < t413;
    let t1846 = t1253 * t1646;
    let t1847 = t1252 * t1846;
    let t1851 = piecewise3::<F>(t418, t1844, -t1844);
    (t1846, t1847, t1851)
}
