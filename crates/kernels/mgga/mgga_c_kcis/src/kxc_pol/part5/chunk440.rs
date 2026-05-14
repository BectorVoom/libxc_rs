//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 440/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk440<F: Float>(t1022: F, t1767: F, t1096: F, t1092: F, t1646: F, t8: F, t168: F) -> (F, F, F, F) {
    let t1768 = t1022 * t1767;
    let t1769 = t1096 * t1768;
    let t1770 = t1092 * t1769;
    let t1772 = t8 * t1646;
    let t1773 = 1.0 + t168 + t1772;
    (t1768, t1769, t1770, t1773)
}
