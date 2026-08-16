//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 804/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk804<F: Float>(t13839: F, t5171: F, t21752: F, t9791: F, t2606: F, t21672: F, t3885: F, t2599: F, t14233: F, t18633: F, t18746: F, t1901: F, t193: F, t21719: F, t21724: F, t21728: F, t21732: F, t21736: F, t21740: F, t21744: F, t21748: F, t21754: F, t446: F, t89: F) -> (F, F, F, F, F, F) {
    let t21757 = t13839 * t5171;
    let t21760 = t9791 * t21752;
    let t21761 = t2606 * t21760;
    let t21764 = t3885 * t21672;
    let t21765 = t2599 * t21764;
    let t21768 = -F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14233 + t89 * t193 * t21719 / F::cast_from(3.0_f64) - t446 * t21724 / F::cast_from(9.0_f64) - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t446 * t21728 - t446 * t21732 / F::cast_from(3.0_f64) - t446 * t21736 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t21740 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18633 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t21744 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t21748 - t18746 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t21754 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t21757 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t21761 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t21765;
    (t21757, t21760, t21761, t21764, t21765, t21768)
}
