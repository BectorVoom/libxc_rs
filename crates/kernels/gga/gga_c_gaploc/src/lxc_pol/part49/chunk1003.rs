//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1003/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1003<F: Float>(t40828: F, t40833: F, t40836: F, t40850: F, t40853: F, t2508: F, t2927: F, t3266: F, t3234: F, t8469: F, t2580: F, t2958: F, t9688: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43205 = F::new(0.1281754371690370714e-2) * t40828;
    let t43206 = F::new(0.2563508743380741428e-2) * t40833;
    let t43207 = F::new(0.64087718584518535698e-3) * t40836;
    let t43208 = F::new(0.1281754371690370714e-2) * t40850;
    let t43209 = F::new(0.64087718584518535698e-3) * t40853;
    let t43212 = F::new(0.76905262301422242837e-2) * t2508 * t3266 * t2927;
    let t43213 = t8469 * t3234;
    let t43216 = F::new(0.15381052460284448567e-1) * t2508 * t2580 * t43213;
    let t43217 = t2958 * t9688;
    (t43205, t43206, t43207, t43208, t43209, t43212, t43213, t43216, t43217)
}
