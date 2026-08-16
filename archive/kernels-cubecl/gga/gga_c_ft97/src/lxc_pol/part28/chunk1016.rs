//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1016/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1016<F: Float>(t137236: F, t3188: F, t136151: F, t32067: F, t34370: F, t379: F, t22952: F, t22958: F, t5617: F, t5691: F, t920: F, t136240: F, t34376: F) -> (F, F, F, F, F, F, F) {
    let t144792 = t137236 * t3188;
    let t144794 = t32067 * t136151 * t144792;
    let t144796 = t34370 * t379;
    let t144798 = t22952 * t22958 * t144796;
    let t144801 = t5691 * t920 * t5617;
    let t144803 = t32067 * t136151 * t144801;
    let t144805 = t136240 * t34376;
    (t144792, t144794, t144796, t144798, t144801, t144803, t144805)
}
