//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1083/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1083(t10143: f64, t1882: f64, t10007: f64, t10039: f64, t10044: f64, t10075: f64, t13885: f64, t14200: f64, t1901: f64, t2373: f64, t2409: f64, t2413: f64, t242: f64, t2568: f64, t2569: f64, t2574: f64, t2579: f64, t2619: f64, t41414: f64, t41435: f64, t42546: f64, t446: f64, t713: f64, t724: f64, t761: f64, t773: f64, t9787: f64) -> f64 {
    let t42557 = t1882 * t10143;
    let t42563 = -8.0_f64 * t1901 * t13885 * t761 * t713 * t10044 - 8.0_f64 / 3.0_f64 * t1901 * t9787 * t10075 + 8.0_f64 / 3.0_f64 * t1901 * t10007 * t2409 * t2579 + 8.0_f64 / 9.0_f64 * t1901 * t14200 * t41435 - 2.0_f64 / 3.0_f64 * t446 * t724 * t2619 * t2413 - 12.0_f64 * t446 * t242 * t41414 + 8.0_f64 / 3.0_f64 * t42546 + 8.0_f64 * t446 * t2574 * t2568 * t2373 * t2569 + 8.0_f64 * t446 * t2574 * t773 * t10039 - 8.0_f64 / 9.0_f64 * t42557 + 4.0_f64 / 3.0_f64 * t446 * t724 * t2619 * t2409;
    t42563
}
