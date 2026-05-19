//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 280/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk280<F: Float>(t237: F, t365: F, t830: F, t855: F, t858: F, t863: F, t872: F, t878: F, t882: F, t891: F, t369: F) -> (F, F, F) {
    let t895 = t237 * (-F::new(0.310907e-1) * t858 * t365 + F::new(1.0) * t863 * t872 + t830 - t855 - F::cast_from(0.19751673498613801407e-1_f64) * t878 + F::cast_from(0.5848223622634646207e0_f64) * t882 * t891);
    let t897 = F::cast_from(0.19751673498613801407e-1_f64) * t237 * t878;
    let t898 = t237 * t369;
    (t895, t897, t898)
}
