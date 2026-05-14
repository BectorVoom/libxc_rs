//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 801/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk801<F: Float>(t11543: F, t5546: F, t11576: F, t5578: F, t1334: F, t5574: F, t3861: F, t1907: F, t3893: F, t3862: F, t5577: F, t11581: F, t3901: F, t5573: F, t3899: F, t11516: F, t1906: F) -> (F, F, F, F, F, F, F, F) {
    let t16251 = 4.0 * t11543 * t5546;
    let t16253 = 0.32163648644302209644e2 * t11576 * t5578;
    let t16254 = t5574 * t1334;
    let t16256 = 4.0 * t3861 * t16254;
    let t16257 = t1907 * t3893;
    let t16259 = 2.0 * t3861 * t16257;
    let t16260 = t5577 * t3862;
    let t16262 = 0.96490945932906628932e2 * t11581 * t16260;
    let t16263 = t5573 * t3901;
    let t16264 = t16263 * t1334;
    let t16266 = 0.32163648644302209644e2 * t3899 * t16264;
    let t16267 = t5577 * t3893;
    let t16269 = 0.16081824322151104822e2 * t3899 * t16267;
    let t16270 = t1906 * t11516;
    (t16251, t16253, t16256, t16259, t16262, t16266, t16269, t16270)
}
