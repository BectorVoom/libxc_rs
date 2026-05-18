//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 797/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk797<F: Float>(t1885: F, t759: F, t761: F, t5693: F, t2106: F, t2134: F, t2105: F, t2009: F, t2029: F, t2901: F, t302: F, t2040: F, t2099: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5694 = t1885 * t759;
    let t5695 = t5694 * t761;
    let t5696 = t5693 * t5695;
    let t5699 = t2134 * t2106;
    let t5700 = t2105 * t5699;
    let t5703 = t2009 * t2029;
    let t5704 = t5703 * t2901;
    let t5705 = t302 * t5704;
    let t5708 = t2099 * t2040;
    (t5694, t5695, t5696, t5699, t5700, t5703, t5704, t5705, t5708)
}
