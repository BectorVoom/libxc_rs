//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1360/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1360<F: Float>(t26750: F, t6332: F, t9491: F, t3508: F, t8275: F, t113497: F, t6333: F, t27416: F, t3785: F, t113364: F, t27073: F, t27207: F, t109297: F, t34858: F, t1517: F, t25308: F) -> (F, F, F, F, F, F, F, F) {
    let t119852 = t9491 * t6332 * t26750;
    let t119854 = t3508 * t8275;
    let t119856 = t113497 * t6333;
    let t119858 = t3785 * t27416;
    let t119860 = t113364 * t27073;
    let t119862 = t9491 * t27207;
    let t119864 = t109297 * t34858;
    let t119866 = t25308 * t1517;
    (t119852, t119854, t119856, t119858, t119860, t119862, t119864, t119866)
}
