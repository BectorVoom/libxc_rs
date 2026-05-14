//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 729/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk729<F: Float>(t1091: F, t18680: F, t2606: F, t1168: F, t4917: F, t9808: F, t3891: F, t13839: F, t5171: F, t9791: F, t21672: F, t3885: F, t2599: F, t14233: F, t18633: F, t18746: F, t1901: F, t193: F, t21719: F, t21724: F, t21728: F, t21732: F, t21736: F, t21740: F, t21744: F, t446: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21747 = t18680 * t1091;
    let t21748 = t2606 * t21747;
    let t21752 = t4917 * t1168;
    let t21753 = t9808 * t21752;
    let t21754 = t3891 * t21753;
    let t21757 = t13839 * t5171;
    let t21760 = t9791 * t21752;
    let t21761 = t2606 * t21760;
    let t21764 = t3885 * t21672;
    let t21765 = t2599 * t21764;
    let t21768 = -4.0 / 9.0 * t14233 + t89 * t193 * t21719 / 3.0 - t446 * t21724 / 9.0 - 10.0 / 81.0 * t446 * t21728 - t446 * t21732 / 3.0 - t446 * t21736 / 3.0 - 2.0 / 9.0 * t446 * t21740 - 2.0 / 3.0 * t18633 + 2.0 / 3.0 * t1901 * t21744 - 2.0 / 3.0 * t1901 * t21748 - t18746 / 3.0 + 2.0 / 9.0 * t1901 * t21754 + 2.0 / 3.0 * t1901 * t21757 - 2.0 / 3.0 * t1901 * t21761 - 2.0 / 3.0 * t1901 * t21765;
    (t21747, t21748, t21753, t21754, t21757, t21760, t21761, t21764, t21765, t21768)
}
