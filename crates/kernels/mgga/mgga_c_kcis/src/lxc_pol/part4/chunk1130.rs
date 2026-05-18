//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1130/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1130<F: Float>(t1646: F, t934: F, t829: F, t14301: F, t10415: F, t1727: F, t3270: F, t10269: F, t10339: F, t10341: F, t10343: F, t10351: F, t10414: F, t1102: F, t14051: F, t14250: F, t14253: F, t14260: F, t14263: F, t14269: F, t14272: F, t14275: F, t14279: F, t14284: F, t14288: F, t14292: F, t14296: F, t14299: F, t278: F, t344: F) -> (F, F, F) {
    let t14302 = t1646 * t934;
    let t14303 = t14302 * t829;
    let t14304 = t14301 * t14303;
    let t14307 = t10415 * t1727;
    let t14308 = t14307 * t3270;
    let t14311 = -F::new(0.21901432222222222221e-2) * t14250 + F::new(0.1478346675e-2) * t344 * t14253 - F::new(0.2920190962962962963e-3) * t10339 + F::new(0.43802864444444444445e-3) * t10341 + F::new(0.73004774074074074075e-3) * t10343 - t14260 - F::new(0.19711289e-2) * t10351 + F::new(0.98556445e-3) * t10414 * t14263 - F::new(4.0) * t278 * t14051 + F::new(0.13140859333333333333e-2) * t10269 * t14269 - F::new(0.32852148333333333333e-3) * t14272 - F::new(0.98556445e-3) * t344 * t14275 + F::new(0.7391733375e-3) * t1102 * t14279 - F::new(0.295669335e-2) * t1102 * t14284 + F::new(0.19711289e-2) * t1102 * t14288 - F::new(0.1478346675e-2) * t1102 * t14292 + F::new(0.39422578e-2) * t1102 * t14296 + F::new(0.21901432222222222222e-3) * t14299 - F::new(0.39422578e-2) * t10414 * t14304 - F::new(0.19711289e-2) * t10414 * t14308;
    (t14302, t14303, t14311)
}
