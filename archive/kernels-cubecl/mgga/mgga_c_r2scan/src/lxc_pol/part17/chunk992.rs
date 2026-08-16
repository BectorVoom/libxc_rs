//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 992/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk992<F: Float>(t11845: F, t10871: F, t10887: F, t10893: F, t10896: F, t10898: F, t10906: F, t11444: F, t11835: F, t11838: F, t11840: F, t12192: F) -> F {
    let t12193 = F::cast_from(0.12805040077930161442e0_f64) * t11845;
    let t12194 = -t10871 - F::cast_from(0.86682217400542685632e-1_f64) * t11835 - F::cast_from(0.86682217400542685632e-1_f64) * t11838 - F::cast_from(0.86682217400542685632e-1_f64) * t11840 + t12192 + t12193 + t10887 + t10893 + t10896 - t10898 - t11444 + t10906;
    t12194
}
