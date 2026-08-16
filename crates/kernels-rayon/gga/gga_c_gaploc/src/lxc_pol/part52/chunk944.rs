//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 944/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk944(t41448: f64, t41477: f64, t2558: f64, t39002: f64, t9647: f64, t12311: f64, t2554: f64, t7064: f64, t1843: f64, t47178: f64, t39040: f64, t5539: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47555 = 0.31952438294933958064e0_f64 * t41448;
    let t47558 = 0.12780975317973583226e0_f64 * t41477;
    let t47594 = t9647 * t39002 * t2558;
    let t47597 = t7064 * t12311 * t2554;
    let t47607 = t9647 * t1843 * t47178;
    let t47610 = t9647 * t5539 * t39040;
    (t47555, t47558, t47594, t47597, t47607, t47610)
}
