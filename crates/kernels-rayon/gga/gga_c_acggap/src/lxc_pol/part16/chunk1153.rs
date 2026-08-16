//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1153/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1153(t2001: f64, t5811: f64, t31160: f64, t31168: f64, t35380: f64, t35385: f64, t35388: f64, t35391: f64, t35393: f64, t35395: f64, t39907: f64, t39910: f64, t39914: f64, t39919: f64, t39923: f64, t39925: f64, t39928: f64, t39930: f64, t39932: f64) -> f64 {
    let t39934 = t2001 * t5811;
    let t39936 = 0.22921875e-1_f64 * t39907 + 0.1528125e-1_f64 * t39910 - 0.17149607247227894789e-2_f64 * t31160 - 0.42874018118069736972e-3_f64 * t39914 - 0.7145669686344956162e-3_f64 * t31168 + 0.32155513588552302729e-2_f64 * t39919 - 0.32155513588552302729e-2_f64 * t39923 - t35380 - 11.0_f64 / 576.0_f64 * t39925 + t35385 + t35388 + t35391 - t35393 + 0.22921875e-1_f64 * t39928 + 0.68598428988911579156e-2_f64 * t39930 - t35395 - 0.34299214494455789578e-2_f64 * t39932 - 0.34299214494455789578e-2_f64 * t39934;
    t39936
}
