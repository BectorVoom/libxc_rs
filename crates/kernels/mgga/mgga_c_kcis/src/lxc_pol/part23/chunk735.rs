//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 735/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk735<F: Float>(t189: F, t8823: F, t2665: F, t850: F, t2683: F, t851: F, t47: F, t8655: F, t8656: F, t8659: F, t680: F, t2372: F, t88: F) -> (F, F, F, F, F, F, F, F) {
    let t8824 = t189 * t8823;
    let t8825 = t2665 * t850;
    let t8826 = t8825 * t2683;
    let t8829 = t8825 * t851;
    let t8832 = t47 * t8655;
    let t8833 = t8656 * t8659;
    let t8836 = t8656 * t680;
    let t8845 = t88 * t2372;
    (t8824, t8825, t8826, t8829, t8832, t8833, t8836, t8845)
}
