//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2851/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2851<F: Float>(t61130: F, t10439: F, t22688: F, t750: F, t49926: F, t18263: F, t4308: F, t49940: F, t23211: F, t72: F, t757: F, t61165: F) -> (F, F, F, F, F, F, F) {
    let t76963 = F::cast_from(12.0_f64) * t61130;
    let t76965 = t10439 * t750 * t22688;
    let t76966 = F::cast_from(24.0_f64) * t76965;
    let t76967 = F::cast_from(0.65061487801810439052e-1_f64) * t49926;
    let t76969 = F::cast_from(12.0_f64) * t18263 * t4308;
    let t76970 = F::cast_from(0.10526802520742363173e2_f64) * t49940;
    let t76972 = t23211 * t72 * t757;
    let t76973 = F::cast_from(0.18311447306006545054e-3_f64) * t76972;
    let t76974 = F::cast_from(36.0_f64) * t61165;
    (t76963, t76966, t76967, t76969, t76970, t76973, t76974)
}
