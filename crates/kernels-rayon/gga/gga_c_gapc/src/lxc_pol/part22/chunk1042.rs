//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1042/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1042(t12023: f64, t12037: f64, t209: f64, t3804: f64, t575: f64, t687: f64, t1049: f64, t10526: f64, t10529: f64, t2967: f64, t3179: f64, t3480: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12038 = t12023 + t12037;
    let t12039 = t12038 * t209;
    let t12040 = t3804 * t575;
    let t12041 = t12040 * t687;
    let t12042 = t10526 * t1049;
    let t12043 = t10529 * t2967;
    let t12044 = 2.0_f64 * t12043;
    let t12045 = t3480 * t3179;
    (t12038, t12039, t12040, t12041, t12042, t12043, t12044, t12045)
}
