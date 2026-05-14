//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 624/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk624<F: Float>(t5193: F, t6661: F, t5192: F, t5182: F, t1849: F, t719: F, t1060: F, t2063: F) -> (F, F, F, F, F) {
    let t6662 = t5193 * t6661;
    let t6663 = t5192 * t6662;
    let t6664 = t5182 * t6663;
    let t6666 = t719 * t1849;
    let t6667 = t2063 * t1060;
    (t6662, t6663, t6664, t6666, t6667)
}
