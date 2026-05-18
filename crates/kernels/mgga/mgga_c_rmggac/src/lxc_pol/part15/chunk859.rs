//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 859/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk859<F: Float>(t9214: F, t9227: F, t9232: F, t9234: F, t9236: F, t9238: F, t38382: F, t38965: F, t39122: F, t39308: F, t40332: F, t40623: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42351 = F::new(0.17025839957319135759e-4) * t9214;
    let t42355 = F::new(0.4726e1) * t9227;
    let t42356 = F::new(0.4726e1) * t9232;
    let t42357 = F::new(0.4726e1) * t9234;
    let t42358 = F::new(0.85129199786595678796e-5) * t9236;
    let t42359 = F::new(0.11974241701863808564e0) * t9238;
    let t42600 = F::new(0.2927036860455597649e0) * t38382;
    let t42793 = F::new(0.66211599834018861287e-4) * t38965;
    let t42856 = F::new(0.66211599834018861287e-4) * t39122;
    let t42913 = F::new(0.66211599834018861287e-4) * t39308;
    let t43375 = F::new(0.58540737209111952978e0) * t40332;
    let t43492 = F::new(0.2927036860455597649e0) * t40623;
    (t42351, t42355, t42356, t42357, t42358, t42359, t42600, t42793, t42856, t42913, t43375, t43492)
}
