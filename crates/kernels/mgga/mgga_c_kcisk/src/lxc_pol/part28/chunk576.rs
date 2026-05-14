//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 576/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk576<F: Float>(t5192: F, t6668: F, t5182: F, t4594: F, t704: F, t1336: F, t140: F) -> (F, F, F, F) {
    let t6669 = t5192 * t6668;
    let t6670 = t5182 * t6669;
    let t6672 = t4594 * t704;
    let t6674 = t140 * t1336 * t6672;
    (t6669, t6670, t6672, t6674)
}
