//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1036/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1036(t7031: f64, t7445: f64, t1860: f64, t12020: f64, t2091: f64, t225: f64, t7910: f64, t7919: f64, t1824: f64, t2085: f64, t1338: f64, t7918: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26959 = t7031 * t7445;
    let t26960 = t1860 * t26959;
    let t26989 = t12020 * t2091;
    let t27009 = t7910 * t225;
    let t27068 = t7919 * t225;
    let t27074 = t2085 * t1824;
    let t27097 = t1338 * t7918;
    (t26959, t26960, t26989, t27009, t27068, t27074, t27097)
}
