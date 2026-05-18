//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1262/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1262<F: Float>(t16090: F, t469: F, t45: F, t5586: F, t4370: F, t5595: F, t1893: F, t3860: F, t3863: F, t11536: F, t1919: F, t11539: F, t3919: F) -> (F, F, F, F, F, F) {
    let t16092 = F::new(0.62182e-1) * t16090 * t469;
    let t16093 = t45 * t5586;
    let t16100 = t5595 * t4370;
    let t16103 = t1893 * t3860;
    let t16105 = F::new(2.0) * t16103 * t3863;
    let t16106 = t11536 * t1919;
    let t16107 = t11539 * t3919;
    (t16092, t16093, t16100, t16105, t16106, t16107)
}
