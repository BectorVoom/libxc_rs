//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 768/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk768<F: Float>(t2105: F, t5699: F, t2009: F, t2029: F, t2901: F, t302: F, t2040: F, t2099: F, t2038: F, t2023: F, t768: F, t46: F, t2037: F, t747: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5700 = t2105 * t5699;
    let t5703 = t2009 * t2029;
    let t5704 = t5703 * t2901;
    let t5705 = t302 * t5704;
    let t5708 = t2099 * t2040;
    let t5709 = t2038 * t5708;
    let t5711 = t768 * t2023;
    let t5712 = t5711 * t46;
    let t5713 = t2037 * t5712;
    let t5716 = t747 * t747;
    let t5717 = 1.0 / t5716;
    (t5700, t5703, t5704, t5705, t5708, t5709, t5711, t5712, t5713, t5716, t5717)
}
