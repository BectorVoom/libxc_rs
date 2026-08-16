//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1215/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1215(t5281: f64, t5341: f64, t1240: f64, t13303: f64, t13312: f64, t15179: f64, t15182: f64, t15189: f64, t15191: f64, t15192: f64, t19661: f64, t19664: f64, t19682: f64, t19686: f64, t19689: f64, t19692: f64, t19696: f64, t19700: f64, t3638: f64, t6843: f64, t9552: f64, t9563: f64) -> (f64, f64) {
    let t20294 = t5281 * t5341;
    let t20309 = -0.34822083333333333332e-2_f64 * t19661 + t15179 + 0.69644166666666666664e-2_f64 * t19664 + 0.13345e0_f64 * t1240 * t20294 - 0.25794135802469135802e-3_f64 * t9552 - t15182 + t15189 + 0.77382407407407407407e-3_f64 * t13303 + t15191 + t15192 - 0.41270617283950617283e-2_f64 * t13312 - 0.25794135802469135802e-3_f64 * t9563 - 0.38691203703703703703e-2_f64 * t19682 - 0.30952962962962962963e-2_f64 * t19686 + 0.46429444444444444444e-2_f64 * t19689 + 0.23214722222222222222e-2_f64 * t19692 + 0.46429444444444444444e-2_f64 * t19696 - 0.15476481481481481481e-2_f64 * t19700 - 0.66725e-1_f64 * t3638 * t6843;
    (t20294, t20309)
}
