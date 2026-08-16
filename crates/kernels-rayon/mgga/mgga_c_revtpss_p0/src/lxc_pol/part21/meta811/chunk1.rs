//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2963/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2963(t1011: f64, t15140: f64, t53972: f64, t11672: f64, t15592: f64, t4915: f64, t4919: f64, t51861: f64, t51865: f64, t51993: f64, t53955: f64, t53958: f64, t53961: f64, t53964: f64, t53967: f64, t53970: f64) -> f64 {
    let t53974 = t1011 * t53972 * t15140;
    let t53987 = -0.95275595817932748826e-4_f64 * t53955 - t53958 / 72.0_f64 - t53961 / 144.0_f64 - t53964 / 36.0_f64 + t53967 / 108.0_f64 + t53970 / 216.0_f64 + 7.0_f64 / 648.0_f64 * t53974 - t1011 * t4915 * t51861 / 48.0_f64 - t1011 * t4915 * t51865 / 48.0_f64 - t1011 * t4919 * t51993 / 12.0_f64 - 0.22866142996303859718e-2_f64 * t11672 * t15592;
    t53987
}
