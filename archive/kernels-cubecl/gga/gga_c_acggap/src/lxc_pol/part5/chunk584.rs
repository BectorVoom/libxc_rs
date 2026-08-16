//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 584/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk584<F: Float>(t3476: F, t409: F, t932: F, t935: F, t322: F, t922: F, t1426: F, t175: F, t384: F, t1137: F, t962: F, t1131: F) -> (F, F, F, F, F, F, F) {
    let t3477 = t3476 * t409;
    let t3479 = t935 * t932;
    let t3491 = t922 * t322;
    let t3493 = t1426 * t175 * t3491;
    let t3494 = t384 * t3493;
    let t3504 = t1137 * t962;
    let t3529 = t1131 * t322;
    (t3477, t3479, t3491, t3493, t3494, t3504, t3529)
}
