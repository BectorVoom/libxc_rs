//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 303/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk303(t1218: f64, t1249: f64, t1253: f64, t1255: f64, t301: f64, t317: f64, t332: f64, t231: f64, t893: f64, t992: f64, t1093: f64, t1190: f64, t902: f64) -> (f64, f64, f64, f64) {
    let t1258 = -t1218 * t317 - t1253 * t301 - 2.0_f64 * t1249 + 2.0_f64 * t1255;
    let t1259 = t1258 * t332;
    let t1263 = t231 * t893 * t992;
    let t1268 = 0.234754e0_f64 * t1190 - t902 - 0.14443083333333333333e0_f64 * t1093;
    (t1258, t1259, t1263, t1268)
}
