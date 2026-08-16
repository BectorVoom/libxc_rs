//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 787/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk787(t5576: f64, t713: f64, t722: f64, t730: f64, t1893: f64, t685: f64, t1855: f64, t1901: f64, t683: f64, t1899: f64, t1478: f64, t154: f64, t277: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5578 = t713 * t5576 * t722;
    let t5580 = 0.5848223622634646207e0_f64 * t730 * t5578;
    let t5581 = t685 * t1893;
    let t5583 = 6.0_f64 * t1855 * t5581;
    let t5585 = t1893 * t1901 * t683;
    let t5587 = 0.48245938496077605201e2_f64 * t1899 * t5585;
    let t5589 = t154 * t1478 * t277;
    (t5578, t5580, t5581, t5583, t5585, t5587, t5589)
}
