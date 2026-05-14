//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 809/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk809<F: Float>(t262: F, t39680: F, t7192: F, t2350: F, t848: F, t8630: F, t833: F, t7198: F, t333: F, t8708: F, t352: F, t7204: F, t1614: F, t2064: F, t903: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t39681 = t262 * t39680;
    let t39682 = t7192 * t39681;
    let t39684 = t2350 * t848;
    let t39685 = t262 * t39684;
    let t39686 = t8630 * t39685;
    let t39688 = t2350 * t833;
    let t39689 = t262 * t39688;
    let t39690 = t7198 * t39689;
    let t39692 = t8708 * t333;
    let t39693 = t262 * t39692;
    let t39694 = t7198 * t39693;
    let t39696 = t8708 * t352;
    let t39697 = t262 * t39696;
    let t39698 = t7204 * t39697;
    let t39700 = t2064 * t1614;
    let t39701 = t903 * t39700;
    (t39681, t39682, t39684, t39685, t39686, t39688, t39689, t39690, t39692, t39693, t39694, t39696, t39697, t39698, t39700, t39701)
}
