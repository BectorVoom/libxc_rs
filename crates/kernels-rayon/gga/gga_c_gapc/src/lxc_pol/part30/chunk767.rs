//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 767/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk767(t1679: f64, t3016: f64, t3013: f64, t5252: f64, t3012: f64, t5248: f64, t1643: f64, t116: f64, t5312: f64, t3708: f64, t5407: f64, t676: f64, t8986: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9106 = t3016 * t1679;
    let t9108 = t5252 * t3013;
    let t9110 = t3012 * t5248;
    let t9111 = t1643 * t9110;
    let t9113 = t116 * t5312;
    let t9114 = t3708 * t5407;
    let t9115 = t9113 * t9114;
    let t9117 = t8986 * t676;
    (t9106, t9108, t9111, t9113, t9115, t9117)
}
