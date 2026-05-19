//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1139/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1139<F: Float>(t278: F, t19160: F, t3202: F, t4554: F, t1646: F, t1704: F, t829: F, t14408: F, t14395: F, t330: F, t1003: F, t14401: F, t19107: F) -> (F, F, F, F, F) {
    let t288 = F::new(0.0) < t278;
    let t19161 = t3202 * t19160;
    let t19162 = t4554 * t19161;
    let t19164 = t1646 * t1704;
    let t19165 = t19164 * t829;
    let t19166 = t14408 * t19165;
    let t19171 = t14395 * t330;
    let t19173 = t19171 * t19164 * t1003;
    let t19176 = t14401 * t19165;
    let t19180 = piecewise3::<F>(t288, t19107, -t19107);
    (t19162, t19166, t19173, t19176, t19180)
}
