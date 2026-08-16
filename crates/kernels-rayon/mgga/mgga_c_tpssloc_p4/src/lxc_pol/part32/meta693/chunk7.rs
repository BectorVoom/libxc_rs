//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2153/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2153(t19761: f64, t1992: f64, t6976: f64, t1825: f64, t22633: f64, t90754: f64, t90818: f64, t26421: f64, t5287: f64, t22751: f64, t28149: f64, t19740: f64, t22897: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97079 = t1992 * t6976 * t19761;
    let t97083 = t22633 * t6976 * t90754 * t1825;
    let t97087 = t22633 * t6976 * t90818 * t1825;
    let t97091 = t22633 * t6976 * t26421 * t5287;
    let t97095 = t22751 * t28149;
    let t97106 = t1992 * t22897 * t19740;
    (t97079, t97083, t97087, t97091, t97095, t97106)
}
