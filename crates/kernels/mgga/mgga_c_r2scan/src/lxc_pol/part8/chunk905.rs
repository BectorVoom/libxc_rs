//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 905/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk905<F: Float>(t2376: F, t818: F, t1004: F, t1275: F, t1248: F, t35: F, t1256: F, t1338: F, t2441: F, t1035: F, t6755: F, t1044: F, t2449: F, t3250: F, t860: F, t1039: F, t2881: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8355 = t2376 * t818;
    let t8358 = t1004 * t1275;
    let t8377 = t1248 * t35;
    let t8385 = t1256 * t35;
    let t8484 = t1338 * t2441;
    let t8487 = t6755 * t1035;
    let t8540 = t2449 * t1044;
    let t8542 = t860 * t3250;
    let t8543 = t1039 * t2881;
    (t8355, t8358, t8377, t8385, t8484, t8487, t8540, t8542, t8543)
}
