//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1014/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1014(t20022: f64, t942: f64, t1901: f64, t1902: f64, t1903: f64, t1909: f64, t20044: f64, t20098: f64, t3194: f64, t446: f64, t452: f64, t4612: f64, t59506: f64, t75048: f64, t75050: f64, t75071: f64, t75115: f64, t75117: f64, t75119: f64, t8210: f64, t85393: f64, t986: f64) -> (f64, f64) {
    let t85740 = t20022 * t942;
    let t85752 = -4.0_f64 / 3.0_f64 * t446 * t452 * t986 * t20098 - 8.0_f64 / 27.0_f64 * t75048 - 4.0_f64 / 9.0_f64 * t75050 + 4.0_f64 / 3.0_f64 * t75071 - 8.0_f64 / 9.0_f64 * t75115 + 8.0_f64 / 9.0_f64 * t75117 + 8.0_f64 / 3.0_f64 * t75119 + 4.0_f64 / 9.0_f64 * t1901 * t1902 * t1903 * t20044 * t942 + 8.0_f64 / 3.0_f64 * t1901 * t1902 * t3194 * t85740 + 8.0_f64 / 3.0_f64 * t1901 * t1909 * t8210 * t85393 + 4.0_f64 / 3.0_f64 * t1901 * t59506 * t4612;
    (t85740, t85752)
}
