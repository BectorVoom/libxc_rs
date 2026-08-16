//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 874/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk874<F: Float>(t11981: F, t1291: F, t9874: F, t25: F, t514: F, t28: F, t517: F, t1376: F, t68: F, t522: F, t9212: F, t9214: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11982 = F::cast_from(96.0_f64) * t11981;
    let t11984 = F::cast_from(0.56968947174242584612e-3_f64) * t1291 * t9874;
    let t11985 = t25 * t25;
    let t11987 = F::cast_from(1.0_f64) / t514 / t11985;
    let t11998 = t28 * t28;
    let t12000 = F::cast_from(1.0_f64) / t517 / t11998;
    let t12019 = t1376 * t1376;
    let t12020 = F::cast_from(1.0_f64) / t12019;
    let t12021 = t68 * t12020;
    let t12044 = F::cast_from(24.0_f64) * t9212 * t522;
    let t12045 = t9214 * t522;
    (t11982, t11984, t11987, t12000, t12019, t12020, t12021, t12044, t12045)
}
