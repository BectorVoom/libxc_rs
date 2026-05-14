//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 291/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk291<F: Float>(t1634: F, t571: F, t311: F, t436: F, t579: F, t657: F, t79: F) -> (F, F, F, F, F) {
    let t1651 = 0.29896666666666666667e0 * t1634;
    let t1653 = f64::sqrt(t571);
    let t1657 = t311 * t436 * t579;
    let t1658 = 0.82156666666666666667e-1 * t1657;
    let t1659 = t79 * t657;
    (t1651, t1653, t1657, t1658, t1659)
}
