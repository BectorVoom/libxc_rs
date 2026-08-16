//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1091/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1091(t13153: f64, t144: f64, t167: f64, t17198: f64, t1901: f64, t20684: f64, t2210: f64, t446: f64, t4462: f64, t569: f64, t574: f64, t77821: f64, t77823: f64, t77868: f64, t78438: f64, t78565: f64, t78573: f64, t85538: f64, t86868: f64, t87097: f64) -> f64 {
    let t87780 = -t446 * t574 * t167 * t86868 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t77821 - 8.0_f64 / 9.0_f64 * t77823 + 8.0_f64 / 3.0_f64 * t446 * t569 * t167 * t85538 + 4.0_f64 / 3.0_f64 * t77868 + 2.0_f64 / 3.0_f64 * t1901 * t2210 * t17198 * t4462 + 4.0_f64 * t446 * t144 * t87097 + 4.0_f64 / 3.0_f64 * t1901 * t13153 * t20684 - 8.0_f64 / 9.0_f64 * t78438 + 8.0_f64 / 9.0_f64 * t78565 + 4.0_f64 / 3.0_f64 * t78573;
    t87780
}
