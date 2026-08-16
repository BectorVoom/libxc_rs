//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1168/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1168(t20947: f64, t221: f64, t25154: f64, t20857: f64, t6605: f64, t9972: f64, t20853: f64, t815: f64, t20944: f64, t81959: f64, t1894: f64, t20756: f64, t236: f64, t81969: f64) -> (f64, f64, f64, f64, f64) {
    let t105345 = t25154 * t221 * t20947;
    let t105348 = t6605 * t9972 * t20857;
    let t105353 = t6605 * t815 * t20853;
    let t105366 = t81959 * t20944;
    let t105370 = t81969 * t1894 * t236 * t20756;
    (t105345, t105348, t105353, t105366, t105370)
}
