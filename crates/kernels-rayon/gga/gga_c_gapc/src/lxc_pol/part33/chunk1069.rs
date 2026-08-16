//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1069/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1069(t12744: f64, t7418: f64, t9709: f64, t126: f64, t190: f64, t3044: f64, t15542: f64, t7953: f64, t21801: f64, t7259: f64, t7325: f64, t11799: f64, t129: f64, t18866: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33284 = t9709 * t12744 * t7418;
    let t33287 = t126 * t190 * t3044;
    let t33289 = t7953 * t33287 * t15542;
    let t33291 = t7259 * t21801;
    let t33292 = t33291 * t7325;
    let t33295 = t18866 * t129 * t11799;
    (t33284, t33287, t33289, t33291, t33292, t33295)
}
