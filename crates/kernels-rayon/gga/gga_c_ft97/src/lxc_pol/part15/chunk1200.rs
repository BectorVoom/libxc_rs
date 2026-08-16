//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1200/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1200(t15128: f64, t22405: f64, t10683: f64, t10703: f64, t1212: f64, t1248: f64, t15402: f64, t1901: f64, t21978: f64, t22346: f64, t296: f64, t319: f64, t4139: f64, t446: f64, t4969: f64, t5225: f64, t5299: f64, t5330: f64, t72523: f64, t835: f64, t840: f64, t84734: f64, t84740: f64, t84767: f64, t871: f64, t88105: f64, t88756: f64) -> (f64, f64) {
    let t91005 = t15128 * t22405;
    let t91015 = -8.0_f64 / 27.0_f64 * t84734 + 4.0_f64 / 9.0_f64 * t84740 + 2.0_f64 / 3.0_f64 * t446 * t835 * t319 * t88756 + 8.0_f64 * t446 * t10683 * t871 * t21978 * t1248 - 12.0_f64 * t446 * t10683 * t319 * t5225 * t5299 + 8.0_f64 / 3.0_f64 * t1901 * t10703 * t4969 * t5330 + 4.0_f64 / 3.0_f64 * t446 * t840 * t871 * t22346 * t1212 + 8.0_f64 * t446 * t296 * t91005 - 8.0_f64 / 9.0_f64 * t84767 + 16.0_f64 / 27.0_f64 * t72523 + 8.0_f64 / 3.0_f64 * t1901 * t4139 * t15402 * t88105;
    (t91005, t91015)
}
