//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1130/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1130<F: Float>(t16270: F, t3862: F, t11513: F, t1907: F, t3899: F, t12780: F, t5618: F, t11388: F, t1919: F, t4374: F, t16144: F, t16048: F, t11409: F, t11411: F, t11413: F, t11415: F, t11455: F, t11457: F, t11460: F, t16050: F, t16062: F, t16088: F) -> (F, F, F, F, F, F) {
    let t16271 = t16270 * t3862;
    let t16273 = 0.51725014705706168417e3 * t11513 * t16271;
    let t16274 = t1907 * t3862;
    let t16276 = 6.0 * t3899 * t16274;
    let t16277 = t5618 * t12780;
    let t16280 = t11388 * t1919;
    let t16281 = t16280 * t4374;
    let t16292 = 0.22076e0 * t16144;
    let t16301 = 0.13418888888888888889e0 * t16048;
    let t16306 = -0.26837777777777777778e0 * t11409 + 0.67094444444444444447e-1 * t11411 - 0.20128333333333333334e0 * t11413 + 0.10064166666666666667e0 * t11415 + 0.60385e0 * t16088 + 0.12077e1 * t16062 + t16301 - 0.40256666666666666667e0 * t16050 - 0.18396666666666666667e0 * t11455 + 0.5519e-1 * t11457 + 0.18396666666666666667e-1 * t11460;
    (t16273, t16276, t16277, t16281, t16292, t16306)
}
