//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 975/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk975(t1310: f64, t1932: f64, t2007: f64, t32161: f64, t32162: f64, t32301: f64, t32303: f64, t32305: f64, t32307: f64, t32309: f64, t32312: f64, t32316: f64, t32320: f64, t32323: f64, t32325: f64, t32329: f64, t32338: f64, t32340: f64, t508: f64, t651: f64, t671: f64, t6983: f64, t6985: f64, t7007: f64, t7221: f64, t8447: f64) -> f64 {
    let t32341 = -t1310 * t8447 - 2.0_f64 * t1932 * t7221 - 2.0_f64 * t2007 * t6983 - t32161 * t508 - 2.0_f64 * t32162 * t671 - 2.0_f64 * t32316 * t651 - 4.0_f64 * t6985 * t7007 - 4.0_f64 * t32301 - 4.0_f64 * t32303 - 4.0_f64 * t32305 - 4.0_f64 * t32307 - 4.0_f64 * t32309 - 4.0_f64 * t32312 - t32320 + 2.0_f64 * t32323 - 4.0_f64 * t32325 + 2.0_f64 * t32329 - t32338 - t32340;
    t32341
}
