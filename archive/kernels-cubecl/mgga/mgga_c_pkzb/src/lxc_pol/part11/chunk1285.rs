//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1285/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1285<F: Float>(t31230: F, t31275: F, t833: F, t852: F, t11264: F, t18612: F, t11311: F, t862: F, t10012: F, t11167: F, t11222: F, t11296: F, t11299: F, t1196: F, t1197: F, t18882: F, t2257: F, t2279: F, t27812: F, t3102: F, t3103: F, t31196: F, t31198: F, t3780: F, t3792: F, t6272: F, t6308: F, t6313: F, t8107: F, t870: F, t872: F, t9875: F, t9958: F) -> (F, F, F) {
    let t31279 = F::cast_from(1.0_f64) * t833 * (t31230 + t31275) * t852;
    let t31281 = F::cast_from(0.51726012919273400301e3_f64) * t18612 * t11264;
    let t31282 = t11311 * t862;
    let t31309 = F::cast_from(0.2069040516770936012e4_f64) * t18882 * t11167 + t31196 - t31198 - t31279 - t31281 + F::cast_from(1.0_f64) * t31282 * t872 - F::cast_from(24.0_f64) * t6313 * t11222 * t870 + F::cast_from(18.0_f64) * t2279 * t3780 * t3102 - F::cast_from(6.0_f64) * t6272 * t11296 - F::cast_from(6.0_f64) * t2257 * t3103 * t3792 - F::cast_from(6.0_f64) * t2257 * t1197 * t9958 + F::cast_from(0.96491876992155210402e2_f64) * t6308 * t11299 + F::cast_from(0.96491876992155210402e2_f64) * t2279 * t27812 * t1196 + F::cast_from(0.96491876992155210402e2_f64) * t2279 * t10012 * t3102 + F::cast_from(0.10526802520742363173e2_f64) * t8107 * t9875;
    (t31279, t31281, t31309)
}
