//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1057/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1057<F: Float>(t1988: F, t8549: F, t1095: F, t1426: F, t34045: F, t598: F, t1980: F, t4806: F, t7476: F, t7799: F, t8555: F, t13287: F, t2302: F, t31195: F, t3196: F) -> (F, F, F, F, F) {
    let t34794 = t1988 * t8549;
    let t34798 = t598 * t1426 * t1095 * t34045;
    let t34802 = t1980 * t7476 * t1095 * t4806;
    let t34804 = t7799 * t8555;
    let t34817 = t31195 * t13287 * t2302 * t3196;
    (t34794, t34798, t34802, t34804, t34817)
}
