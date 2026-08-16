//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1468/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1468(t121142: f64, t121144: f64, t121159: f64, t121160: f64, t121162: f64, t121165: f64, t121169: f64, t121174: f64, t121177: f64, t122917: f64, t124728: f64, t2040: f64, t27863: f64, t32679: f64, t34170: f64, t4034: f64, t672: f64, t7057: f64) -> f64 {
    let t124933 = -2.0_f64 * t122917 * t2040 - 2.0_f64 * t124728 * t672 - 2.0_f64 * t27863 * t7057 - 2.0_f64 * t34170 * t4034 + t121142 - t121144 - t121159 + t121160 - t121162 - t121165 - t121169 - t121174 + t121177 - t32679;
    t124933
}
