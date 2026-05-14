//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 919/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk919<F: Float>(t31066: F, t3539: F, t2191: F, t3544: F, t7744: F, t2075: F, t7897: F, t30273: F, t5907: F, t19434: F, t7877: F, t3564: F, t1428: F, t30605: F, t457: F, t12872: F, t30892: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31067 = t3539 * t31066;
    let t31071 = t3544 * t7744 * t2191;
    let t31074 = t2075 * t7897;
    let t31075 = t3544 * t31074;
    let t31078 = t5907 * t30273;
    let t31081 = t19434 * t7877;
    let t31082 = t3564 * t31081;
    let t31089 = t1428 * t30605;
    let t31090 = t457 * t31089;
    let t31093 = t12872 * t30892;
    (t31067, t31071, t31075, t31078, t31081, t31082, t31089, t31090, t31093)
}
