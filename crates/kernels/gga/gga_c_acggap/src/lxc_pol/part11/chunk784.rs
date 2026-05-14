//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 784/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk784<F: Float>(t1165: F, t16020: F, t604: F, t7346: F, t1160: F, t7432: F, t7365: F, t16548: F, t1992: F, t7585: F, t7842: F, t930: F, t2067: F, t4180: F, t7836: F, t3427: F, t7647: F) -> (F, F, F, F, F, F, F, F) {
    let t30099 = t7346 * t1165 * t604 * t16020;
    let t30105 = t1160 * t7432;
    let t30106 = t30105 * t7365;
    let t30110 = t7346 * t1165 * t604 * t16548;
    let t30118 = t7585 * t7842 * t1992 * t930;
    let t30120 = t4180 * t2067;
    let t30121 = t30120 * t7836;
    let t30123 = t7647 * t3427;
    (t30099, t30105, t30106, t30110, t30118, t30120, t30121, t30123)
}
