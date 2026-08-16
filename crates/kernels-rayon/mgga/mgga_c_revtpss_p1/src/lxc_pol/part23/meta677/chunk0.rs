//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2414/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2414(t13041: f64, t44173: f64, t13061: f64, t13100: f64, t828: f64, t12879: f64, t1247: f64, t1251: f64, t42994: f64, t1231: f64, t12898: f64, t43813: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44174 = t44173 * t13041;
    let t44202 = t44173 * t13061;
    let t44225 = t828 * t13100;
    let t44250 = t828 * t12879;
    let t44264 = t1247 * t42994 * t1251;
    let t44291 = t1231 * t12898;
    let t44307 = 0.86419753086419753087e-1_f64 * t43813;
    (t44174, t44202, t44225, t44250, t44264, t44291, t44307)
}
