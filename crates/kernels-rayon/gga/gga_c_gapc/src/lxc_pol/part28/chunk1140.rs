//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1140/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1140(t33273: f64, t7967: f64, t961: f64, t11853: f64, t2578: f64, t7199: f64, t12744: f64, t7418: f64, t9709: f64, t126: f64, t190: f64, t3044: f64) -> (f64, f64, f64, f64) {
    let t33275 = t7967 * t33273 * t961;
    let t33278 = t2578 * t7199 * t11853;
    let t33284 = t9709 * t12744 * t7418;
    let t33287 = t126 * t190 * t3044;
    (t33275, t33278, t33284, t33287)
}
