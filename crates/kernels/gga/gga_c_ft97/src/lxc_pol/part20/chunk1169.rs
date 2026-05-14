//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1169/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1169<F: Float>(t255: F, t6074: F, t7514: F, t53798: F, t6161: F, t6942: F, t8232: F, t10007: F, t111085: F, t111089: F, t11593: F, t13702: F, t13706: F, t13757: F, t13852: F, t14167: F, t14175: F, t14188: F, t1901: F, t2405: F, t2413: F, t24569: F, t24599: F, t2469: F, t2606: F, t28108: F, t28267: F, t28356: F, t3746: F, t42334: F, t42362: F, t446: F, t51687: F, t53797: F, t54032: F, t684: F, t6861: F, t6930: F, t729: F, t9787: F, t98123: F) -> (F,) {
    let t111356 = t7514 * t255 * t6074;
    let t111363 = t53798 * t6161;
    let t111389 = t8232 * t6942;
    let t111395 = 2.0 / 9.0 * t1901 * t42334 * t24569 * t13852 - 2.0 / 9.0 * t1901 * t14175 * t6930 * t2413 - 8.0 / 27.0 * t54032 * t111085 * t13706 - 4.0 / 27.0 * t54032 * t111089 * t14188 + 4.0 / 3.0 * t53797 * t111356 * t13757 + 4.0 / 9.0 * t53797 * t98123 * t14167 + 4.0 / 9.0 * t53797 * t111363 * t13702 - 4.0 / 27.0 * t54032 * t111363 * t13706 - 2.0 / 27.0 * t1901 * t42362 * t6861 * t2405 - 2.0 / 9.0 * t1901 * t10007 * t28267 * t684 - 4.0 / 27.0 * t1901 * t51687 * t6930 * t2405 + 2.0 / 3.0 * t446 * t729 * t2469 * t28108 + 2.0 / 9.0 * t1901 * t9787 * t28356 - 4.0 / 27.0 * t111389 - 4.0 / 9.0 * t11593 * t2606 * t24599 * t3746;
    (t111395,)
}
