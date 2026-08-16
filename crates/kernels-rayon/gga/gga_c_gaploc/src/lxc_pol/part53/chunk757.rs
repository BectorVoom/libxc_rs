//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 757/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk757(t21502: f64, t28668: f64, t2012: f64, t7809: f64, t2530: f64, t299: f64, t1890: f64, t27997: f64, t7802: f64, t296: f64, t9688: f64, t1: f64, t787: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28669 = t21502 * t28668;
    let t28673 = t2012 * t7809;
    let t28703 = t299 * t2530;
    let t28720 = t1890 * t27997;
    let t28737 = t2012 * t7802;
    let t28844 = t296 * t9688;
    let t28846 = t787 * t28844 * t1;
    (t28669, t28673, t28703, t28720, t28737, t28846)
}
