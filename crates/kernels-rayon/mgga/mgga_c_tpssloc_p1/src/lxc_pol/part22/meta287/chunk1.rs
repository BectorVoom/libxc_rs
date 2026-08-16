//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1441/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1441(t13258: f64, t4184: f64, t242: f64, t9972: f64, t812: f64, t2639: f64, t4236: f64, t1512: f64, t9674: f64, t2638: f64, t4166: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13260 = 7.0_f64 / 1152.0_f64 * t13258 * t4184;
    let t13261 = t9972 * t242;
    let t13262 = t812 * t13261;
    let t13275 = 7.0_f64 / 2304.0_f64 * t2639 * t4236;
    let t13277 = 7.0_f64 / 2304.0_f64 * t9674 * t1512;
    let t13278 = t4166 * t2638;
    (t13260, t13261, t13262, t13275, t13277, t13278)
}
