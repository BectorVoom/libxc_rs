//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1204/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1204(t2586: f64, t41146: f64, t59162: f64, t59135: f64, t9523: f64, t1516: f64, t47275: f64, t5628: f64, t9601: f64, t5619: f64, t9671: f64, t16673: f64, t2638: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t59221 = t2586 * t41146 * t59162;
    let t59224 = t2586 * t9523 * t59135;
    let t59259 = t47275 * t1516;
    let t59263 = t9601 * t5628;
    let t59276 = t9671 * t5619;
    let t59281 = t16673 * t2638;
    (t59221, t59224, t59259, t59263, t59276, t59281)
}
