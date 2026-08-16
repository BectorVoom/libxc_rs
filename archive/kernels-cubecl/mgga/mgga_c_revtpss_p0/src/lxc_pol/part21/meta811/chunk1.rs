//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2963/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2963<F: Float>(t1011: F, t15140: F, t53972: F, t11672: F, t15592: F, t4915: F, t4919: F, t51861: F, t51865: F, t51993: F, t53955: F, t53958: F, t53961: F, t53964: F, t53967: F, t53970: F) -> F {
    let t53974 = t1011 * t53972 * t15140;
    let t53987 = -F::cast_from(0.95275595817932748826e-4_f64) * t53955 - t53958 / F::cast_from(72.0_f64) - t53961 / F::cast_from(144.0_f64) - t53964 / F::cast_from(36.0_f64) + t53967 / F::cast_from(108.0_f64) + t53970 / F::cast_from(216.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t53974 - t1011 * t4915 * t51861 / F::cast_from(48.0_f64) - t1011 * t4915 * t51865 / F::cast_from(48.0_f64) - t1011 * t4919 * t51993 / F::cast_from(12.0_f64) - F::cast_from(0.22866142996303859718e-2_f64) * t11672 * t15592;
    t53987
}
