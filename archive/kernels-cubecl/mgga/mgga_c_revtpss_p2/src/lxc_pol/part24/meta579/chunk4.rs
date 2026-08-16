//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1789/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1789<F: Float>(t58777: F, t70942: F, t83699: F, t83719: F, t83731: F, t83735: F, t83748: F, t83751: F, t83758: F, t83783: F, t83798: F, t6573: F, t6587: F) -> (F, F) {
    let t91260 = -F::cast_from(11.0_f64) / F::cast_from(81.0_f64) * t70942 + t83699 / F::cast_from(27.0_f64) + t83719 / F::cast_from(54.0_f64) - F::cast_from(0.57927562257303111285e-1_f64) * t83731 - F::cast_from(0.57165357490759649296e-3_f64) * t83735 - F::cast_from(0.17149607247227894789e-2_f64) * t83748 + F::cast_from(0.18292914397043087775e-1_f64) * t83751 - F::cast_from(0.16937883700965822013e-3_f64) * t58777 + F::cast_from(0.22866142996303859718e-2_f64) * t83758 - F::cast_from(0.22866142996303859718e-2_f64) * t83783 + F::cast_from(0.34299214494455789578e-2_f64) * t83798;
    let t91272 = t6573 * t6587;
    (t91260, t91272)
}
