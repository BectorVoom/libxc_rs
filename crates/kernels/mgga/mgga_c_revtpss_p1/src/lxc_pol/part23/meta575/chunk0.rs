//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2182/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2182<F: Float>(t45: F, t57: F, t14441: F, t10446: F, t22671: F, t22688: F, t4377: F, t5825: F, t78: F, t10457: F, t4384: F, t81: F, t162: F, t187: F, zeta_threshold: F) -> (F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t23193 = F::new(12.0) * t14441;
    let t23201 = piecewise3::<F>(t151, F::new(0.0), -F::new(8.0) / F::new(27.0) * t10446 * t22688 + F::new(4.0) / F::new(3.0) * t4377 * t5825 + F::new(4.0) / F::new(3.0) * t78 * t22671);
    let t23209 = piecewise3::<F>(t155, F::new(0.0), F::new(8.0) / F::new(27.0) * t10457 * t22688 + F::new(4.0) / F::new(3.0) * t4384 * t5825 - F::new(4.0) / F::new(3.0) * t81 * t22671);
    let t23210 = t23201 + t23209;
    let t23211 = t23210 * t162;
    let t23213 = F::cast_from(0.19751673498613801407e-1_f64) * t23211 * t187;
    (t23193, t23210, t23211, t23213)
}
