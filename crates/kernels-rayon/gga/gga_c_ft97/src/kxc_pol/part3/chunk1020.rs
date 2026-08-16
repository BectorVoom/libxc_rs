//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1020/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1020(t19782: f64, t295: f64, t312: f64, t15290: f64, t19006: f64, t1901: f64, t193: f64, t19587: f64, t19590: f64, t19594: f64, t19598: f64, t19602: f64, t19606: f64, t19610: f64, t19614: f64, t19618: f64, t19623: f64, t19627: f64, t19631: f64, t19635: f64, t3281: f64, t446: f64, t89: f64) -> f64 {
    let t19784 = t295 * t19782 * t312;
    let t19788 = t15290 * t19006;
    let t19791 = -2.0_f64 / 9.0_f64 * t1901 * t19587 + 2.0_f64 / 9.0_f64 * t1901 * t19590 - 2.0_f64 / 9.0_f64 * t1901 * t19594 - 2.0_f64 / 9.0_f64 * t446 * t19598 + 4.0_f64 / 9.0_f64 * t3281 * t19602 - t446 * t19606 / 9.0_f64 - t446 * t19610 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t19614 - 4.0_f64 / 9.0_f64 * t1901 * t19618 - 4.0_f64 / 9.0_f64 * t1901 * t19623 + 4.0_f64 / 27.0_f64 * t1901 * t19627 - 2.0_f64 / 9.0_f64 * t1901 * t19631 - t19635 / 9.0_f64 + t89 * t193 * t19784 / 3.0_f64 + 4.0_f64 / 27.0_f64 * t1901 * t19788;
    t19791
}
