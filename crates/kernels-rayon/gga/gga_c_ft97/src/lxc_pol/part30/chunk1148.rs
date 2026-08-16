//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1148/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1148(t1253: f64, t7584: f64, t36093: f64, t6213: f64, t10248: f64, t153681: f64, t153684: f64, t153687: f64, t153689: f64, t153696: f64, t153698: f64, t2665: f64, t28987: f64, t33808: f64, t34312: f64, t36057: f64, t4135: f64, t6210: f64, t6216: f64, t684: f64, t6972: f64, t7684: f64) -> f64 {
    let t153705 = t7584 * t1253;
    let t153710 = t36093 * t6213;
    let t153712 = -t34312 * t6972 / 3.0_f64 + t153681 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t153684 - t4135 * t7684 - 12.0_f64 * t153687 - t6216 * t2665 * t153689 * t684 / 9.0_f64 - t33808 * t28987 / 18.0_f64 + t153696 / 54.0_f64 - t6216 * t2665 * t153698 * t684 / 9.0_f64 - t6210 * t36057 / 3.0_f64 + t6216 * t10248 * t153705 * t684 / 9.0_f64 - t153710 / 18.0_f64;
    t153712
}
