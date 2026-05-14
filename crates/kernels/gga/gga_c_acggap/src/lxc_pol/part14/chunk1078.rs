//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1078/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1078<F: Float>(t1734: F, t694: F, t7278: F, t301: F, t9476: F, t2354: F, t3952: F, t105: F, t1680: F, t2166: F, t24794: F, t24811: F, t2541: F, t28242: F, t36617: F, t36619: F, t38615: F, t38641: F, t38665: F, t38693: F, t40604: F, t40635: F, t40666: F, t40695: F, t40729: F, t40771: F, t40791: F, t40815: F, t40837: F, t40860: F, t40880: F, t40907: F, t40939: F, t469: F, t567: F, t7297: F, t8372: F, t8382: F, t8387: F, t9082: F, t9096: F, t9097: F, t9098: F, t9806: F) -> (F,) {
    let t40948 = t694 * t7278 * t1734;
    let t40955 = t9476 * t301;
    let t40959 = t2354 * t3952;
    let t40969 = 6.0 * t567 * t8387 * t8382 - t36617 + t36619 - 2.0 * t567 * t9082 * t1680 + 2.0 * t38615 + t567 * t105 * (t38641 + t38665 + t38693 + t40604 + t40635 + t40666 + t40695 + t40729 + t40771 + t40791 + t40815 + t40837 + t40860 + t40880 + t40907 + t40939) * t469 + 3.0 * t40948 - t567 * t9806 * t2166 - 6.0 * t7297 * t2541 * t28242 - 12.0 * t8372 * t2541 * t40955 + 4.0 * t9096 * t40959 * t9098 + 2.0 * t9096 * t9097 * t24794 - 3.0 * t7297 * t2541 * t24811;
    (t40969,)
}
