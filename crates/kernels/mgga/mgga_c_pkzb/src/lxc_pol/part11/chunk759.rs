//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 759/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk759<F: Float>(t2029: F, t759: F, t178: F, t5711: F, t2020: F, t2902: F, t774: F, t2899: F, t2911: F, t5974: F, t2104: F, t2924: F, t2922: F, t2037: F, t5953: F, t7663: F) -> (F, F, F, F, F, F, F) {
    let t7701 = t2029 * t759;
    let t7706 = t5711 * t178;
    let t7707 = t2020 * t7706;
    let t7710 = t774 * t2902;
    let t7712 = 0.57165357490759649296e-3 * t2899 * t7710;
    let t7713 = t5974 * t2911;
    let t7715 = 0.57165357490759649296e-3 * t2104 * t7713;
    let t7716 = t774 * t2924;
    let t7718 = 0.28582678745379824648e-3 * t2922 * t7716;
    let t7725 = t2037 * t7706;
    let t7736 = t5953 * t7663;
    (t7701, t7707, t7712, t7715, t7718, t7725, t7736)
}
