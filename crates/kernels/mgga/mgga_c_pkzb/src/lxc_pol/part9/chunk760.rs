//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 760/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk760<F: Float>(t5576: F, t713: F, t722: F, t730: F, t1893: F, t685: F, t1855: F, t1901: F, t683: F, t1899: F, t1478: F, t154: F, t277: F, t276: F, t275: F, t4784: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5578 = t713 * t5576 * t722;
    let t5580 = 0.5848223622634646207e0 * t730 * t5578;
    let t5581 = t685 * t1893;
    let t5583 = 6.0 * t1855 * t5581;
    let t5585 = t1893 * t1901 * t683;
    let t5587 = 0.48245938496077605201e2 * t1899 * t5585;
    let t5589 = t154 * t1478 * t277;
    let t5591 = 5.0 / 1296.0 * t276 * t5589;
    let t5592 = t4784 * t275;
    (t5578, t5580, t5581, t5583, t5585, t5587, t5589, t5591, t5592)
}
