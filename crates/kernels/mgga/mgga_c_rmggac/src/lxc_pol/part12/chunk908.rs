//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 908/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk908<F: Float>(t262: F, t39692: F, t7198: F, t352: F, t8708: F, t7204: F, t1614: F, t2064: F, t903: F, t1679: F, t7203: F, t7206: F) -> (F, F, F, F, F, F, F, F) {
    let t39693 = t262 * t39692;
    let t39694 = t7198 * t39693;
    let t39696 = t8708 * t352;
    let t39697 = t262 * t39696;
    let t39698 = t7204 * t39697;
    let t39700 = t2064 * t1614;
    let t39701 = t903 * t39700;
    let t39702 = F::new(0.23948483403727617128e0) * t39701;
    let t39705 = t1679 * t7203;
    let t39706 = t39705 * t7206;
    (t39693, t39694, t39696, t39697, t39698, t39700, t39702, t39706)
}
