//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 958/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk958<F: Float>(t1988: F, t8549: F, t1095: F, t1426: F, t34045: F, t598: F, t1980: F, t4806: F, t7476: F, t7799: F, t8555: F, t13287: F, t2302: F, t31195: F, t3196: F, t17912: F, t31443: F, t3176: F) -> (F, F, F, F, F, F) {
    let t34794 = t1988 * t8549;
    let t34795 = 0.15724046144802076034e-2 * t34794;
    let t34798 = t598 * t1426 * t1095 * t34045;
    let t34802 = t1980 * t7476 * t1095 * t4806;
    let t34803 = 0.10482697429868050689e-2 * t34802;
    let t34804 = t7799 * t8555;
    let t34817 = t31195 * t13287 * t2302 * t3196;
    let t34821 = t31443 * t17912 * t2302 * t3176;
    (t34795, t34798, t34803, t34804, t34817, t34821)
}
