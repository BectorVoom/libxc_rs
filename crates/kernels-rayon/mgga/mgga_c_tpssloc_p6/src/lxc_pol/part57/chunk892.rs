//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 892/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk892(t112: f64, t33578: f64, t1873: f64, t27188: f64, t33234: f64, t7042: f64, t7467: f64, t2039: f64, t33211: f64, t88: f64, t7801: f64, t8601: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33579 = t33578 * t112;
    let t33583 = 2.0_f64 * t27188 * t1873;
    let t33585 = 2.0_f64 * t33234 * t1873;
    let t33587 = 2.0_f64 * t7042 * t7467;
    let t33595 = 2.0_f64 * t33211 * t2039;
    let t33596 = t88 * t7467;
    let t33598 = 2.0_f64 * t33596 * t2039;
    let t33600 = 2.0_f64 * t8601 * t7801;
    (t33579, t33583, t33585, t33587, t33595, t33596, t33598, t33600)
}
