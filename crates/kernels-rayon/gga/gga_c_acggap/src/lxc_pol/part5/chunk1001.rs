//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1001/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1001(t12586: f64, t5147: f64, t1017: f64, t1163: f64, t1165: f64, t1539: f64, t4313: f64, t12589: f64, t5152: f64, t1413: f64, t3740: f64, t1008: f64, t4344: f64) -> (f64, f64, f64, f64, f64) {
    let t16781 = t12586 * t5147;
    let t16786 = t1163 * t1165 * t4313 * t1539 * t1017;
    let t16788 = t12589 * t5152;
    let t16792 = t3740 * t1413;
    let t16794 = t1008 * t4344;
    (t16781, t16786, t16788, t16792, t16794)
}
