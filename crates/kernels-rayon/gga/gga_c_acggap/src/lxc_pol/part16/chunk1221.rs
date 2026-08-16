//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1221/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1221(t1734: f64, t694: f64, t7278: f64, t301: f64, t9476: f64, t2354: f64, t3952: f64, t105: f64, t1680: f64, t2166: f64, t24794: f64, t24811: f64, t2541: f64, t28242: f64, t36617: f64, t36619: f64, t38615: f64, t38641: f64, t38665: f64, t38693: f64, t40604: f64, t40635: f64, t40666: f64, t40695: f64, t40729: f64, t40771: f64, t40791: f64, t40815: f64, t40837: f64, t40860: f64, t40880: f64, t40907: f64, t40939: f64, t469: f64, t567: f64, t7297: f64, t8372: f64, t8382: f64, t8387: f64, t9082: f64, t9096: f64, t9097: f64, t9098: f64, t9806: f64) -> f64 {
    let t40948 = t694 * t7278 * t1734;
    let t40955 = t9476 * t301;
    let t40959 = t2354 * t3952;
    let t40969 = 6.0_f64 * t567 * t8387 * t8382 - t36617 + t36619 - 2.0_f64 * t567 * t9082 * t1680 + 2.0_f64 * t38615 + t567 * t105 * (t38641 + t38665 + t38693 + t40604 + t40635 + t40666 + t40695 + t40729 + t40771 + t40791 + t40815 + t40837 + t40860 + t40880 + t40907 + t40939) * t469 + 3.0_f64 * t40948 - t567 * t9806 * t2166 - 6.0_f64 * t7297 * t2541 * t28242 - 12.0_f64 * t8372 * t2541 * t40955 + 4.0_f64 * t9096 * t40959 * t9098 + 2.0_f64 * t9096 * t9097 * t24794 - 3.0_f64 * t7297 * t2541 * t24811;
    t40969
}
