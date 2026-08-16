//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 528/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk528(t1628: f64, t3181: f64, t3172: f64, t1589: f64, t3137: f64, t3133: f64, t2293: f64, t2416: f64, t1445: f64, t447: f64, t9171: f64, t590: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9511 = t1628 * t3181;
    let t9514 = t1628 * t3172;
    let t9517 = t1589 * t3137;
    let t9520 = t1589 * t3133;
    let t9523 = t2416 * t2293;
    let t9524 = t1445 * t9523;
    let t9527 = t9171 * t447;
    let t9528 = t1445 * t9527;
    let t9531 = t3133 * t590;
    (t9511, t9514, t9517, t9520, t9524, t9528, t9531)
}
