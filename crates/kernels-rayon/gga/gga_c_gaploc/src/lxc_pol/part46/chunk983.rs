//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 983/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk983(t1457: f64, t2103: f64, t43316: f64, t13024: f64, t5771: f64, t13016: f64, t8638: f64, t1645: f64, t3025: f64, t9689: f64, t13020: f64, t1: f64, t106: f64, t13096: f64, t316: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43690 = t2103 * t1457 * t43316;
    let t43693 = 0.71500979903700853338e0_f64 * t5771 * t13024;
    let t43695 = 0.10725146985555128001e1_f64 * t8638 * t13016;
    let t43698 = 0.10725146985555128001e1_f64 * t3025 * t1645 * t9689;
    let t43699 = t5771 * t13020;
    let t43703 = t13096 * t1 * t106 * t316;
    (t43690, t43693, t43695, t43698, t43699, t43703)
}
