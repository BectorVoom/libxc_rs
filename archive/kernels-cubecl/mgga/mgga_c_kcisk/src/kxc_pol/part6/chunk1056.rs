//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1056/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1056<F: Float>(t31283: F, t4203: F, t14374: F, t30489: F, t498: F, t493: F, t2271: F, t8279: F, t30962: F, t6322: F, t4230: F, t31133: F, t470: F) -> (F, F, F, F, F) {
    let t31284 = t4203 * t31283;
    let t31286 = t14374 * t30489;
    let t31287 = t498 * t31286;
    let t31288 = t493 * t31287;
    let t31290 = t2271 * t8279;
    let t31292 = t6322 * t30962;
    let t31293 = t4230 * t31292;
    let t31295 = t470 * t31133;
    (t31284, t31288, t31290, t31293, t31295)
}
