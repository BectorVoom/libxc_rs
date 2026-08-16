//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1523/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1523(t17691: f64, t4588: f64, t4582: f64, t14187: f64, t17686: f64, t5878: f64, t884: f64, t3071: f64, t1616: f64, t4347: f64, t376: f64, t5866: f64) -> (f64, f64, f64, f64, f64) {
    let t17692 = t4588 * t17691;
    let t17693 = t4582 * t17692;
    let t17696 = t14187 * t17686;
    let t17697 = t4582 * t17696;
    let t17700 = t5878 * t884;
    let t17701 = t3071 * t17700;
    let t17704 = t1616 * t4347;
    let t17705 = t3071 * t17704;
    let t17712 = t376 * t5866;
    (t17693, t17697, t17701, t17705, t17712)
}
