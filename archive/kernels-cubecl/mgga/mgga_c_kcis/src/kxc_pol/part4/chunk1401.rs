//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1401/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1401<F: Float>(t4463: F, t6177: F, t6176: F, t4426: F, t6141: F, t25: F, t494: F, t6178: F, t1599: F, t12651: F, t2104: F, t4457: F) -> (F, F, F, F) {
    let t18200 = t6177 * t4463;
    let t18201 = t6176 * t18200;
    let t18205 = t6141 * t4426 / F::cast_from(324.0_f64);
    let t18210 = t25 * t494;
    let t18211 = t18210 * t6178;
    let t18213 = t1599 * t18211 / F::cast_from(144.0_f64);
    let t18217 = t12651 * t2104 * t4457;
    (t18201, t18205, t18213, t18217)
}
