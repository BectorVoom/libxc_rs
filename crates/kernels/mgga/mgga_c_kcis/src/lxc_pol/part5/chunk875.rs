//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 875/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk875<F: Float>(t3786: F, t7237: F, t1477: F, t6964: F, t542: F, t3814: F, t7122: F, t1482: F, t7141: F, t1924: F, t1102: F, t344: F, t3743: F, t486: F, t5423: F, t5449: F, t5486: F, t7028: F, t7214: F, t7218: F, t7222: F, t7226: F, t7230: F, t7234: F) -> (F, F, F, F, F, F, F, F) {
    let t7238 = t3786 * t7237;
    let t7241 = t1477 * t6964;
    let t7242 = t542 * t7241;
    let t7245 = t3814 * t7122;
    let t7246 = t542 * t7245;
    let t7249 = t1482 * t7141;
    let t7250 = t542 * t7249;
    let t7253 = t1924 * t1924;
    let t7257 = -t3743 + F::cast_from(0.8760572888888888889e-3_f64) * t5423 + F::new(0.19711289e-2) * t5449 - F::cast_from(0.13140859333333333333e-2_f64) * t5486 + F::cast_from(0.10950716111111111111e-2_f64) * t1102 * t7214 + F::new(0.19711289e-2) * t1102 * t7218 - F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t7222 - F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t7226 + F::cast_from(0.65704296666666666667e-3_f64) * t1102 * t7230 + F::cast_from(0.7391733375e-3_f64) * t344 * t7234 - F::cast_from(0.295669335e-2_f64) * t1102 * t7238 + F::cast_from(0.1478346675e-2_f64) * t344 * t7242 + F::new(0.19711289e-2) * t344 * t7246 - F::new(0.98556445e-3) * t344 * t7250 - F::new(4.0) * t7253 - F::new(4.0) * t486 * t7028;
    (t7238, t7241, t7242, t7245, t7246, t7249, t7250, t7257)
}
