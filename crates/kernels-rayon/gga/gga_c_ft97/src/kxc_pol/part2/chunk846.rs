//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 846/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk846(t1023: f64, t1058: f64, t12278: f64, t12597: f64, t12600: f64, t12603: f64, t12606: f64, t13245: f64, t165: f64, t1953: f64, t2081: f64, t2228: f64, t3414: f64, t3588: f64, t564: f64, t614: f64) -> f64 {
    let t13246 = -t1023 * t2228 - t1058 * t1953 - t1058 * t2081 - t12597 * t165 - 2.0_f64 * t3414 * t614 - 2.0_f64 * t3588 * t564 - 4.0_f64 * t12278 + 4.0_f64 * t12600 + 8.0_f64 * t12603 - 12.0_f64 * t12606 + t13245;
    t13246
}
