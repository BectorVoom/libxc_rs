//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1285/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1285(t31230: f64, t31275: f64, t833: f64, t852: f64, t11264: f64, t18612: f64, t11311: f64, t862: f64, t10012: f64, t11167: f64, t11222: f64, t11296: f64, t11299: f64, t1196: f64, t1197: f64, t18882: f64, t2257: f64, t2279: f64, t27812: f64, t3102: f64, t3103: f64, t31196: f64, t31198: f64, t3780: f64, t3792: f64, t6272: f64, t6308: f64, t6313: f64, t8107: f64, t870: f64, t872: f64, t9875: f64, t9958: f64) -> (f64, f64, f64) {
    let t31279 = 1.0_f64 * t833 * (t31230 + t31275) * t852;
    let t31281 = 0.51726012919273400301e3_f64 * t18612 * t11264;
    let t31282 = t11311 * t862;
    let t31309 = 0.2069040516770936012e4_f64 * t18882 * t11167 + t31196 - t31198 - t31279 - t31281 + 1.0_f64 * t31282 * t872 - 24.0_f64 * t6313 * t11222 * t870 + 18.0_f64 * t2279 * t3780 * t3102 - 6.0_f64 * t6272 * t11296 - 6.0_f64 * t2257 * t3103 * t3792 - 6.0_f64 * t2257 * t1197 * t9958 + 0.96491876992155210402e2_f64 * t6308 * t11299 + 0.96491876992155210402e2_f64 * t2279 * t27812 * t1196 + 0.96491876992155210402e2_f64 * t2279 * t10012 * t3102 + 0.10526802520742363173e2_f64 * t8107 * t9875;
    (t31279, t31281, t31309)
}
