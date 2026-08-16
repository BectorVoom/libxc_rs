//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 746/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk746(t1860: f64, t26959: f64, t26198: f64, t12020: f64, t2091: f64, t26200: f64, t225: f64, t7910: f64, t26231: f64, t26251: f64, t26255: f64, t26266: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26960 = t1860 * t26959;
    let t26988 = 0.16449340668482264365e-1_f64 * t26198;
    let t26989 = t12020 * t2091;
    let t26993 = 0.38381794893125283518e-1_f64 * t26200;
    let t27009 = t7910 * t225;
    let t27012 = 7.0_f64 / 1152.0_f64 * t26231;
    let t27019 = 7.0_f64 / 1152.0_f64 * t26251;
    let t27022 = 7.0_f64 / 288.0_f64 * t26255;
    let t27027 = 7.0_f64 / 72.0_f64 * t26266;
    (t26960, t26988, t26989, t26993, t27009, t27012, t27019, t27022, t27027)
}
