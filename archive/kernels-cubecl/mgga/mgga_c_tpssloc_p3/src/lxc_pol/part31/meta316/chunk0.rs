//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1207/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1207<F: Float>(t28: F, t517: F, t1376: F, t68: F, t522: F, t9212: F, t9214: F, t3824: F, t592: F, t1285: F, t2221: F, t1287: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11998 = t28 * t28;
    let t12000 = F::cast_from(1.0_f64) / t517 / t11998;
    let t12019 = t1376 * t1376;
    let t12020 = F::cast_from(1.0_f64) / t12019;
    let t12021 = t68 * t12020;
    let t12044 = F::cast_from(24.0_f64) * t9212 * t522;
    let t12045 = t9214 * t522;
    let t12048 = F::cast_from(12.0_f64) * t592 * t3824;
    let t12050 = t2221 * t1285;
    let t12052 = t2221 * t1287;
    (t12000, t12019, t12020, t12021, t12044, t12045, t12048, t12050, t12052)
}
