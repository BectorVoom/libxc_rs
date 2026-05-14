//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 714/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk714<F: Float>(t1783: F, t8820: F, t1310: F, t2448: F, t2455: F, t652: F, t742: F) -> (F, F, F, F) {
    let t8821 = t1783 * t8820;
    let t8822 = t1310 * t8821;
    let t8825 = t2448 * t2455;
    let t8831 = 1.0 / t652 / t742;
    (t8821, t8822, t8825, t8831)
}
