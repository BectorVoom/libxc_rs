//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 297/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk297(t106: f64, t2405: f64, t192: f64, t524: f64, t529: f64, t901: f64, t1457: f64, t2335: f64, t1564: f64, t874: f64, t475: f64, t1445: f64) -> (f64, f64, f64, f64, f64) {
    let t2406 = t2405 * t106;
    let t2407 = t2406 * t192;
    let t2410 = t524 * t529;
    let t2411 = t2410 * t901;
    let t2413 = t1457 * t2335;
    let t2416 = t1564 * t874;
    let t2417 = t2416 * t475;
    let t2418 = t1445 * t2417;
    (t2407, t2411, t2413, t2416, t2418)
}
