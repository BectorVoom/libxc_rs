//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1220/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1220<F: Float>(t10434: F, t1945: F, t3926: F, t6296: F, t10457: F, t10481: F, t10487: F, t10492: F, t10498: F, t1937: F, t21608: F, t25201: F, t29484: F, t29507: F, t29528: F, t3104: F, t3110: F, t3116: F, t3931: F, t6257: F, t6285: F, t82: F, t8385: F, t8386: F, t8392: F, t8405: F, t8413: F) -> (F,) {
    let t29594 = t1945 * t10434;
    let t29599 = t6296 * t3926;
    let t29622 = -4.0 * t1937 * t10434 * t3104 - 2.0 * t10481 * t6257 - 5.0 / 2.0 * t6285 * t3926 * t8386 + t29594 * t3104 / 2.0 + t10487 * t6257 / 4.0 + t29599 * t8386 / 8.0 + t3110 * t25201 / 2.0 + t10492 * t6257 / 8.0 + t21608 * t3931 * t8386 / 16.0 - 2.0 * t10498 * t25201 - t8413 * t10457 - 2.0 * t3116 * t29507 + t8405 * t29528 / 2.0 + 30.0 * t8385 * t29528 - 10.0 * t8392 * t29528 - 8.0 * t29484 * t82;
    (t29622,)
}
