//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 411/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk411<F: Float>(t24: F, t494: F, t209: F, t617: F, t612: F, t611: F, t68: F, t610: F) -> (F, F, F, F) {
    let t1593 = t24 * t494;
    let t1595 = t209 * t1593 * t617;
    let t1597 = t612 * t1595 / 576.0;
    let t1598 = t611 * t68;
    let t1599 = t610 * t1598;
    (t1593, t1595, t1597, t1599)
}
