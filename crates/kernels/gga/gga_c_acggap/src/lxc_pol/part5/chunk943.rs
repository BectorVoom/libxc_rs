//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 943/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk943<F: Float>(t2709: F, t2712: F, t2716: F, t2736: F, t2978: F, t2982: F, t2988: F, t2805: F, t2811: F, t2994: F, t1364: F, t228: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14717 = F::cast_from(0.28493333333333333333e0_f64) * t2709;
    let t14718 = F::new(0.2137e0) * t2712;
    let t14719 = F::cast_from(0.34367190188705947438e1_f64) * t2716;
    let t14720 = F::new(0.4274e0) * t2736;
    let t14725 = F::new(480.0) * t2978;
    let t14726 = F::cast_from(0.23392894490538584828e1_f64) * t2982;
    let t14729 = F::cast_from(0.14035736694323150897e2_f64) * t2988;
    let t14731 = F::cast_from(0.2069040516770936012e4_f64) * t2805;
    let t14732 = F::cast_from(0.3859675079686208416e3_f64) * t2811;
    let t14734 = F::new(240.0) * t2994;
    let t14810 = F::new(32.0) * t1364 * t228;
    (t14717, t14718, t14719, t14720, t14725, t14726, t14729, t14731, t14732, t14734, t14810)
}
