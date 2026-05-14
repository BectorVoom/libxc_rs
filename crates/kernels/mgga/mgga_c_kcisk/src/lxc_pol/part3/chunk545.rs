//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 545/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk545<F: Float>(t1899: F, t4803: F, t1800: F, t1869: F, t3805: F, t721: F, t140: F, t3737: F, t673: F) -> (F, F, F, F, F, F) {
    let t4804 = t1899 * t4803;
    let t4805 = t1800 * t4804;
    let t4806 = t1869 * t4805;
    let t4808 = t3805 * t721;
    let t4809 = 0.55273148148148148147e-3 * t4808;
    let t4811 = t140 * t3737 * t673;
    (t4804, t4805, t4806, t4808, t4809, t4811)
}
