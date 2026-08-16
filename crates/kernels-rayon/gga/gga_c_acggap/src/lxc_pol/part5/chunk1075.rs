//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1075/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1075(t394: f64, t5299: f64, t377: f64, t4238: f64, t3077: f64, t4211: f64, t1629: f64, t16539: f64, t3088: f64, t5316: f64, t1160: f64, t407: f64, t545: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19135 = t394 * t5299;
    let t19144 = t377 * t4238;
    let t19149 = t3077 * t4211;
    let t19152 = t3088 * t1629 * t16539;
    let t19161 = t3077 * t5316;
    let t19172 = t1160 * t545 * t879 * t407;
    (t19135, t19144, t19149, t19152, t19161, t19172)
}
