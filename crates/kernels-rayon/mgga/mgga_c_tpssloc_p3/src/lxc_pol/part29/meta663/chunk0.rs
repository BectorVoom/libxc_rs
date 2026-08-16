//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2204/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2204(t26411: f64, t6914: f64, t12420: f64, t26331: f64, t5335: f64, t6976: f64, t1351: f64, t1992: f64, t5318: f64, t550: f64, t16036: f64, t22633: f64, t3807: f64) -> (f64, f64, f64, f64) {
    let t90759 = t6914 * t26411;
    let t90760 = 0.38381794893125283518e-1_f64 * t90759;
    let t90763 = t26331 * t6976 * t5335 * t12420;
    let t90770 = t1992 * t6976 * t5318 * t1351 * t550;
    let t90774 = t22633 * t6976 * t16036 * t3807;
    (t90760, t90763, t90770, t90774)
}
