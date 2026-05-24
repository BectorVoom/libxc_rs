//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 336/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk336<F: Float>(t1663: F, t1664: F, t1645: F, t1634: F, t1638: F) -> (F, F, F) {
    let t1665 = t1663 * t1664;
    let t1667 = F::new(1.0) * t1645 * t1665;
    let t1668 = F::cast_from(0.92708333333333333333e-2_f64) * t1634;
    let t1670 = -t1668 - F::cast_from(0.92708333333333333333e-2_f64) * t1638;
    (t1665, t1667, t1670)
}
