//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1080/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1080(t17999: f64, t197: f64, t2030: f64, t17928: f64, t5951: f64, t1478: f64, t301: f64, t154: f64, t276: f64, t655: f64, t486: f64, t779: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18000 = t17999 * t197;
    let t18002 = t2030 * t2030;
    let t18008 = t17928 * t5951;
    let t18009 = t18008 * t197;
    let t18060 = t1478 * t301;
    let t18063 = t276 * t154 * t18060 * t655;
    let t18086 = t486 * t779;
    (t18000, t18002, t18008, t18009, t18060, t18063, t18086)
}
