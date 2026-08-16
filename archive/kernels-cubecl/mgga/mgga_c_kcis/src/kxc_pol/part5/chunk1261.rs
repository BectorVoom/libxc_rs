//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1261/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1261<F: Float>(t1464: F, t21047: F, t16617: F, t5875: F, t1395: F, t17298: F, t5638: F, t1307: F, t7282: F, t4162: F, t4160: F, t1365: F, t7054: F) -> (F, F, F, F, F) {
    let t21048 = t1464 * t21047;
    let t21050 = t16617 * t5875;
    let t21051 = t1395 * t21050;
    let t21052 = t1464 * t21051;
    let t21055 = t17298 * t5638;
    let t21057 = t7282 * t1307;
    let t21058 = t4162 * t21057;
    let t21059 = t4160 * t21058;
    let t21061 = t7054 * t1365;
    (t21048, t21052, t21055, t21059, t21061)
}
