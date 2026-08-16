//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 907/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk907(t2526: f64, t3977: f64, t242: f64, t10153: f64, t1168: f64, t13952: f64, t13955: f64, t13959: f64, t13961: f64, t13963: f64, t13965: f64, t13967: f64, t14014: f64, t14018: f64, t14020: f64, t14022: f64, t14026: f64, t14030: f64, t1901: f64, t446: f64) -> (f64, f64, f64) {
    let t14033 = t3977 * t2526;
    let t14034 = t242 * t14033;
    let t14037 = t10153 * t1168;
    let t14038 = t242 * t14037;
    let t14041 = 2.0_f64 / 9.0_f64 * t1901 * t13952 + 2.0_f64 / 9.0_f64 * t1901 * t13955 - t13959 - t13961 - t13963 + t13965 - 2.0_f64 / 3.0_f64 * t446 * t13967 - t446 * t14014 / 3.0_f64 - t14018 - t14020 - t446 * t14022 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t14026 - 2.0_f64 / 9.0_f64 * t446 * t14030 - t446 * t14034 / 3.0_f64 - t446 * t14038 / 3.0_f64;
    (t14033, t14037, t14041)
}
