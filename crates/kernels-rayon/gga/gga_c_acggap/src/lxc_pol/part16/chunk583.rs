//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 583/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk583(t381: f64, t5304: f64, t322: f64, t545: f64, t407: f64, t1160: f64, t1655: f64, t310: f64, t547: f64, t848: f64, t449: f64, t556: f64, t864: f64) -> (f64, f64, f64, f64, f64) {
    let t5305 = t381 * t5304;
    let t5315 = t545 * t322;
    let t5316 = t5315 * t407;
    let t5318 = 0.13170898365871023197e1_f64 * t1160 * t5316;
    let t5327 = 0.13170898365871023197e1_f64 * t310 * t1655;
    let t5346 = t848 * t547;
    let t5351 = t449 * t556 * t864;
    (t5305, t5318, t5327, t5346, t5351)
}
