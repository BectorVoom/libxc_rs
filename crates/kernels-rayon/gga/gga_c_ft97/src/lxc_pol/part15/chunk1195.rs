//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1195/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1195(t4917: f64, t5309: f64, t15195: f64, t15460: f64, t1901: f64, t19500: f64, t19506: f64, t22187: f64, t22245: f64, t22368: f64, t22405: f64, t2862: f64, t296: f64, t319: f64, t4139: f64, t44280: f64, t44566: f64, t446: f64, t5225: f64, t5424: f64, t55937: f64, t84167: f64, t84169: f64, t84222: f64, t90308: f64, t90481: f64) -> (f64, f64) {
    let t90717 = t4917 * t5309;
    let t90729 = 8.0_f64 * t446 * t44280 * t319 * t90308 + 4.0_f64 * t446 * t2862 * t5424 * t5225 + 8.0_f64 / 3.0_f64 * t84167 - 4.0_f64 / 9.0_f64 * t84169 + 8.0_f64 / 9.0_f64 * t1901 * t55937 * t22368 + 8.0_f64 / 9.0_f64 * t1901 * t19500 * t22187 + 8.0_f64 / 9.0_f64 * t84222 - t446 * t296 * t90481 / 3.0_f64 - 8.0_f64 / 9.0_f64 * t1901 * t4139 * t44566 * t90717 - 8.0_f64 * t1901 * t15460 * t19506 * t22405 - 8.0_f64 / 3.0_f64 * t1901 * t15195 * t22245;
    (t90717, t90729)
}
