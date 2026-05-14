//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1218/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1218<F: Float>(t43: F, t29410: F, t3085: F, t10434: F, t81: F, t10448: F, t10457: F, t10492: F, t1205: F, t1917: F, t1924: F, t1929: F, t21561: F, t25201: F, t3089: F, t3926: F, t3931: F, t3948: F, t615: F, t617: F, t6257: F, t6265: F, t8344: F, t8357: F, t8360: F, t8385: F, t8386: F) -> (F, F, F, F) {
    let t45 = 0.135e1 < t43;
    let t29466 = piecewise3(t45, 0.0, t29410);
    let t29484 = t3085 * t3085;
    let t29507 = t81 * t10434;
    let t29517 = -t617 * t29466 - t1924 * t29484 * t81 + 4.0 * t1929 * t29484 + 7.0 / 2.0 * t3948 * t6257 + 15.0 / 4.0 * t10492 * t8386 - t8385 * t25201 - t10448 * t6257 / 4.0 - t21561 * t3931 * t8386 / 8.0 - 6.0 * t6265 * t3931 * t1917 + 4.0 * t1929 * t1205 * t8344 - t8357 * t10457 / 2.0 - t3089 * t29507 - t8360 * t10457 / 4.0 + 4.0 * t1929 * t10434 * t615 + 2.0 * t1929 * t3926 * t1917;
    (t29466, t29484, t29507, t29517)
}
