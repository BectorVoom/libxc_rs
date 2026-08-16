//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1206/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1206(t4275: f64, t5474: f64, t14849: f64, t15107: f64, t28141: f64, t47871: f64, t21887: f64, t21891: f64, t21895: f64, t21899: f64, t21903: f64, t21907: f64, t28175: f64, t28181: f64, t37228: f64, t37258: f64) -> (f64, f64, f64, f64, f64) {
    let t55797 = t5474 * t4275;
    let t55816 = t14849 * t15107;
    let t55862 = 4.0_f64 * t28141;
    let t55875 = 4.0_f64 * t47871;
    let t55876 = 70.0_f64 / 3.0_f64 * t37228 + 140.0_f64 / 3.0_f64 * t28175 - 1820.0_f64 / 27.0_f64 * t28181 + t21887 + t21891 + t21895 - t21899 - t21903 - t21907 - 14.0_f64 * t37258 + t55875;
    (t55797, t55816, t55862, t55875, t55876)
}
