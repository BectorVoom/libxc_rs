//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3195/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3195<F: Float>(t3718: F, t44546: F, t5347: F, t12916: F, t17785: F, t5331: F, t3650: F, t5390: F, t12915: F, t16775: F, t247: F, t5384: F) -> (F, F, F, F) {
    let t58850 = t3718 * t44546 * t5347;
    let t58851 = F::cast_from(0.14291339372689912324e-3_f64) * t58850;
    let t58853 = t5331 * t12916 * t17785;
    let t58863 = t3650 * t5390;
    let t58868 = t5384 * t247 * t12915 * t16775;
    (t58851, t58853, t58863, t58868)
}
