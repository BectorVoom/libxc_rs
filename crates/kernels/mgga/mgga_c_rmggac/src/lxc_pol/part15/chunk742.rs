//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 742/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk742<F: Float>(t38471: F, t7473: F, t2320: F, t36520: F, t2310: F, t7921: F, t118: F, t1986: F, t495: F, t571: F, t2001: F, t498: F, t1618: F, t1600: F, t7487: F, t8352: F) -> (F, F, F, F, F, F, F, F) {
    let t40661 = t38471 * t7473;
    let t40679 = t36520 * t2320;
    let t40681 = t7921 * t2310;
    let t40694 = t1986 * t118 * t571 * t495;
    let t40699 = t2001 * t118 * t571 * t498;
    let t40702 = t1986 * t1618;
    let t40705 = t1986 * t1600;
    let t40715 = t7487 * t8352;
    (t40661, t40679, t40681, t40694, t40699, t40702, t40705, t40715)
}
