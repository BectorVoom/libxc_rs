//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 545/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk545<F: Float>(t1426: F, t175: F, t3491: F, t384: F, t1137: F, t962: F, t1131: F, t322: F, t1095: F, t398: F, t177: F, t414: F, t980: F) -> (F, F, F, F, F, F, F) {
    let t3493 = t1426 * t175 * t3491;
    let t3494 = t384 * t3493;
    let t3504 = t1137 * t962;
    let t3529 = t1131 * t322;
    let t3531 = t398 * t1095 * t3529;
    let t3532 = t384 * t3531;
    let t3551 = F::cast_from(0.30011812682648815881e-2_f64) * t980 * t414 * t177;
    (t3493, t3494, t3504, t3529, t3531, t3532, t3551)
}
