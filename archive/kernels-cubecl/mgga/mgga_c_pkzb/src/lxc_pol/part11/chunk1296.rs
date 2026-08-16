//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1296/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1296<F: Float>(t22820: F, t3740: F, t11234: F, t6137: F, t11163: F, t881: F, t18427: F, t18445: F, t18765: F, t18766: F, t27262: F, t27292: F, t27295: F, t31067: F, t31088: F, t31204: F, t31206: F, t31208: F, t31210: F, t31213: F, t31216: F, t31218: F, t31220: F, t31222: F, t31225: F) -> (F, F, F, F) {
    let t31521 = F::cast_from(6.0_f64) * t22820 * t3740;
    let t31523 = F::cast_from(6.0_f64) * t6137 * t11234;
    let t31524 = t11163 * t881;
    let t31558 = t18765 - F::cast_from(0.16068111111111111111e1_f64) * t18427 + t18766 - F::cast_from(0.1549425e1_f64) * t27262 + F::cast_from(0.104195e1_f64) * t27292 + F::cast_from(0.20659e1_f64) * t27295 - F::cast_from(0.6618234375e1_f64) * t31204 + F::cast_from(0.794188125e1_f64) * t31206 - F::cast_from(0.52945875e1_f64) * t31208 - F::cast_from(0.52945875e1_f64) * t31210 - F::cast_from(0.17648625e1_f64) * t31213 + F::cast_from(0.2366859375e0_f64) * t31216 - F::cast_from(0.473371875e0_f64) * t31218 + F::cast_from(0.94674375e0_f64) * t31220 + F::cast_from(0.94674375e0_f64) * t31222 + F::cast_from(0.31558125e0_f64) * t31225 - F::cast_from(0.516475e0_f64) * t31067 + F::cast_from(0.1549425e1_f64) * t31088 - F::cast_from(0.92617777777777777776e0_f64) * t18445;
    (t31521, t31523, t31524, t31558)
}
