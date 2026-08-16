//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 797/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk797(t2771: f64, t6785: f64, t23696: f64, t23661: f64, t3188: f64, t1945: f64, t3120: f64, t1060: f64, t23571: f64, t383: f64, t23384: f64, t6787: f64) -> (f64, f64, f64, f64, f64) {
    let t23697 = t6785 * t2771;
    let t23698 = t23696 * t23697;
    let t23701 = t23661 * t3188;
    let t23704 = t1945 * t3120;
    let t23705 = t23704 * t1060;
    let t23707 = t383 * t23571;
    let t23712 = t23384 * t6787;
    (t23698, t23701, t23705, t23707, t23712)
}
