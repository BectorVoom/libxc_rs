//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2972/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2972(t11875: f64, t11922: f64, t15898: f64, t1011: f64, t16003: f64, t16006: f64, t3241: f64, t42712: f64, t42716: f64, t42719: f64, t42724: f64, t42727: f64, t42740: f64, t42745: f64, t4919: f64, t51873: f64) -> f64 {
    let t54187 = t11875 * t11922 * t15898;
    let t54195 = 2.0_f64 / 9.0_f64 * t3241 * t16003 + t1011 * t4919 * t51873 / 6.0_f64 - 2.0_f64 / 27.0_f64 * t3241 * t16006 + 0.42874018118069736972e-3_f64 * t54187 + t42712 / 81.0_f64 + 5.0_f64 / 1296.0_f64 * t42716 + t42719 / 216.0_f64 + 11.0_f64 / 324.0_f64 * t42724 + t42727 / 144.0_f64 - 5.0_f64 / 162.0_f64 * t42740 - t42745;
    t54195
}
