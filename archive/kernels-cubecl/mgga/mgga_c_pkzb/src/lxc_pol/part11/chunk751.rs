//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 751/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk751<F: Float>(t339: F, t346: F, t6087: F, t336: F, t218: F, t344: F, t5555: F, t1878: F, t847: F, t2238: F, t831: F, t338: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6149 = F::cast_from(1.0_f64) / t339 / t346 / F::cast_from(4.0_f64);
    let t6156 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t6087;
    let t6161 = F::cast_from(0.93011851851851851854e0_f64) * t6087;
    let t6165 = F::cast_from(1.0_f64)/pow_3_2::<F>(t336);
    let t6174 = t218 * t5555 * t344;
    let t6175 = F::cast_from(0.36514074074074074075e0_f64) * t6174;
    let t6177 = t218 * t1878 * t847;
    let t6198 = F::cast_from(1.0_f64) / t2238 / t831;
    let t6199 = t338 * t6198;
    (t6149, t6156, t6161, t6165, t6174, t6175, t6177, t6198, t6199)
}
