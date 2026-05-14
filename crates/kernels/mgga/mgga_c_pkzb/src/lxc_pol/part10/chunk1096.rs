//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1096/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1096<F: Float>(t3740: F, t6317: F, t2192: F, t3766: F, t3743: F, t6149: F, t836: F, t3041: F, t3046: F, t2203: F, t3747: F, t204: F, t3730: F, t648: F) -> (F, F, F, F, F, F, F, F) {
    let t9768 = 2.0 * t6317 * t3740;
    let t9770 = 1.0 * t2192 * t3766;
    let t9771 = t6149 * t3743;
    let t9772 = t9771 * t836;
    let t9774 = t3041 * t3046;
    let t9776 = t2203 * t3747;
    let t9777 = t9776 * t836;
    let t9782 = t204 * t648 * t3730;
    (t9768, t9770, t9771, t9772, t9774, t9776, t9777, t9782)
}
