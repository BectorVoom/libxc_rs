//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 252/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk252<F: Float>(t1648: F, t1653: F, t311: F, t436: F, t579: F, t657: F, t79: F) -> (F, F, F, F) {
    let t1654 = t1653 * t1648;
    let t1657 = t311 * t436 * t579;
    let t1658 = 0.82156666666666666667e-1 * t1657;
    let t1659 = t79 * t657;
    (t1654, t1657, t1658, t1659)
}
