//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1221/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1221<F: Float>(t39882: F, t39886: F, t39894: F, t37937: F, t37940: F, t37947: F, t37951: F, t37954: F, t37957: F, t39874: F, t39879: F, t39891: F) -> F {
    let t41600 = F::new(0.45022119329691164871e0) * t39882;
    let t41601 = F::new(0.19514881078765566037e-1) * t39886;
    let t41605 = F::new(0.93149212406257582492e-1) * t39894;
    let t41606 = F::new(0.95219938395347901946e-2) * t37937 + F::new(0.5200933044032561138e0) * t39874 + F::new(0.28565981518604370584e-1) * t37940 + F::new(0.62295486109113302474e-1) * t37947 + F::new(0.18688645832733990742e0) * t37951 + F::new(0.43663693315433241794e-2) * t39879 + t41600 - t41601 + F::new(0.14282990759302185292e-1) * t37954 + F::new(0.47609969197673950973e-2) * t37957 + F::new(0.43663693315433241794e-2) * t39891 - t41605;
    t41606
}
