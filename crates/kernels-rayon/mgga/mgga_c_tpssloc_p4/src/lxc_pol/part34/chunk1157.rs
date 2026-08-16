//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1157/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1157(t225: f64, t29099: f64, t29071: f64, t29040: f64, t814: f64, t2047: f64, t5611: f64, t26959: f64, t7428: f64, t27979: f64, t7032: f64, t1860: f64, t27956: f64, t7031: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t101509 = t29099 * t225;
    let t101593 = t29071 * t225;
    let t101694 = t814 * t29040;
    let t101708 = t2047 * t5611;
    let t102137 = t7428 * t26959;
    let t102139 = t27979 * t7032;
    let t102142 = t1860 * t7031 * t27956;
    (t101509, t101593, t101694, t101708, t102137, t102139, t102142)
}
