//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 941/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk941(t1646: f64, t934: f64, t829: f64, t14301: f64, t10415: f64, t1727: f64, t3270: f64, t10269: f64, t10339: f64, t10341: f64, t10343: f64, t10351: f64, t10414: f64, t1102: f64, t14051: f64, t14250: f64, t14253: f64, t14260: f64, t14263: f64, t14269: f64, t14272: f64, t14275: f64, t14279: f64, t14284: f64, t14288: f64, t14292: f64, t14296: f64, t14299: f64, t278: f64, t344: f64) -> (f64, f64, f64) {
    let t14302 = t1646 * t934;
    let t14303 = t14302 * t829;
    let t14304 = t14301 * t14303;
    let t14307 = t10415 * t1727;
    let t14308 = t14307 * t3270;
    let t14311 = -0.21901432222222222221e-2_f64 * t14250 + 0.1478346675e-2_f64 * t344 * t14253 - 0.2920190962962962963e-3_f64 * t10339 + 0.43802864444444444445e-3_f64 * t10341 + 0.73004774074074074075e-3_f64 * t10343 - t14260 - 0.19711289e-2_f64 * t10351 + 0.98556445e-3_f64 * t10414 * t14263 - 4.0_f64 * t278 * t14051 + 0.13140859333333333333e-2_f64 * t10269 * t14269 - 0.32852148333333333333e-3_f64 * t14272 - 0.98556445e-3_f64 * t344 * t14275 + 0.7391733375e-3_f64 * t1102 * t14279 - 0.295669335e-2_f64 * t1102 * t14284 + 0.19711289e-2_f64 * t1102 * t14288 - 0.1478346675e-2_f64 * t1102 * t14292 + 0.39422578e-2_f64 * t1102 * t14296 + 0.21901432222222222222e-3_f64 * t14299 - 0.39422578e-2_f64 * t10414 * t14304 - 0.19711289e-2_f64 * t10414 * t14308;
    (t14302, t14303, t14311)
}
