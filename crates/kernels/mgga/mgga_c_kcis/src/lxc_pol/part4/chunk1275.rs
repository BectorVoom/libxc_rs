//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1275/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1275<F: Float>(t11388: F, t1919: F, t4374: F, t16144: F, t16048: F, t11409: F, t11411: F, t11413: F, t11415: F, t11455: F, t11457: F, t11460: F, t16050: F, t16062: F, t16088: F) -> (F, F, F) {
    let t16280 = t11388 * t1919;
    let t16281 = t16280 * t4374;
    let t16292 = F::new(0.22076e0) * t16144;
    let t16301 = F::new(0.13418888888888888889e0) * t16048;
    let t16306 = -F::new(0.26837777777777777778e0) * t11409 + F::new(0.67094444444444444447e-1) * t11411 - F::new(0.20128333333333333334e0) * t11413 + F::new(0.10064166666666666667e0) * t11415 + F::new(0.60385e0) * t16088 + F::new(0.12077e1) * t16062 + t16301 - F::new(0.40256666666666666667e0) * t16050 - F::new(0.18396666666666666667e0) * t11455 + F::new(0.5519e-1) * t11457 + F::new(0.18396666666666666667e-1) * t11460;
    (t16281, t16292, t16306)
}
