//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 585/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk585<F: Float>(t5203: F, t6707: F, t1800: F, t1869: F, t140: F, t1797: F, t5598: F) -> (F, F, F, F) {
    let t6708 = t5203 * t6707;
    let t6709 = t1800 * t6708;
    let t6710 = t1869 * t6709;
    let t6713 = t140 * t5598 * t1797;
    (t6708, t6709, t6710, t6713)
}
