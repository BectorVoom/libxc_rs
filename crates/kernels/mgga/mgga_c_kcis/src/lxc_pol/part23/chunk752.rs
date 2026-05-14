//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 752/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk752<F: Float>(t12833: F, t1599: F, t209: F, t494: F, t617: F, t736: F, t612: F, t110: F, t1611: F, t1607: F, t3970: F) -> (F, F, F, F) {
    let t12834 = t1599 * t12833;
    let t12838 = t209 * t736 * t494 * t617;
    let t12840 = 5.0 / 2592.0 * t612 * t12838;
    let t12841 = t110 * t1611;
    let t12842 = t1599 * t12841;
    let t12844 = t3970 * t1607;
    (t12834, t12840, t12842, t12844)
}
