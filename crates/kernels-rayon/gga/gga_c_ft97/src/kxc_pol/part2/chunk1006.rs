//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 1006/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk1006(t299: f64, t14568: f64, t15554: f64, t10947: f64, t10948: f64, t10949: f64, t10950: f64, t12091: f64, t13: f64, t13291: f64, t14389: f64, t1939: f64, t2316: f64, t2635: f64, t2973: f64) -> f64 {
    let t300 = 10000000.0_f64 <= t299;
    let t15556 = piecewise3(t300, 0.0_f64, t14568 + t15554);
    let tv3rho31 = t1939 + t2316 + t2635 + t2973 + t10947 + t10948 + t10949 + t10950 + t13 * (t12091 + t13291 + t14389 + t15556);
    tv3rho31
}
