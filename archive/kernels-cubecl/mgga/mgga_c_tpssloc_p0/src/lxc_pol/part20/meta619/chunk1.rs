//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2232/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2232<F: Float>(t40745: F, t12908: F, t12924: F, t4101: F, t9912: F, t40754: F, t40761: F, t1409: F, t2516: F, t4194: F, t607: F, t40767: F) -> (F, F, F, F, F, F, F) {
    let t46282 = F::cast_from(0.32530743900905219526e-1_f64) * t40745;
    let t46283 = t12908 * t12924;
    let t46284 = F::cast_from(72.0_f64) * t46283;
    let t46285 = t9912 * t4101;
    let t46286 = F::cast_from(12.0_f64) * t46285;
    let t46287 = F::cast_from(0.31168546390226634765e3_f64) * t40754;
    let t46288 = F::cast_from(0.30762056574649219973e4_f64) * t40761;
    let t46291 = t4194 * t2516 * t1409 * t607;
    let t46292 = F::cast_from(36.0_f64) * t46291;
    let t46293 = F::cast_from(24.0_f64) * t40767;
    (t46282, t46284, t46286, t46287, t46288, t46292, t46293)
}
