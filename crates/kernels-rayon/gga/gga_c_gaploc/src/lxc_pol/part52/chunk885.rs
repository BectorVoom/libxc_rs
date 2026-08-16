//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 885/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk885(t2033: f64, t2365: f64, t35451: f64, t11784: f64, t2679: f64, t9800: f64, t2617: f64, t3626: f64, t7810: f64, t3614: f64, t5241: f64, t9805: f64) -> (f64, f64, f64, f64) {
    let t45819 = t2033 * t2365 * t35451;
    let t45820 = 0.44688112439813033337e-1_f64 * t45819;
    let t45822 = t9800 * t11784 * t2679;
    let t45823 = 0.9585731488480187419e0_f64 * t45822;
    let t45826 = t7810 * t3626 * t2617;
    let t45828 = t5241 * t3614;
    let t45830 = t9805 * t45828 * t2679;
    (t45820, t45823, t45826, t45830)
}
