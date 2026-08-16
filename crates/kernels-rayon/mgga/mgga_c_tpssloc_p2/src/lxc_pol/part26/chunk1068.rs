//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1068/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1068(t15904: f64, t8643: f64, t22574: f64, t3701: f64, t3914: f64, t2019: f64, t1983: f64, t6996: f64, t6999: f64, t1390: f64, t3719: f64, t6878: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22575 = t8643 * t15904;
    let t22577 = 6.0_f64 * t22574 * t22575;
    let t22578 = t3701 * t3914;
    let t22579 = t2019 * t22578;
    let t22580 = t1983 * t22579;
    let t22581 = t6996 * t6999;
    let t22583 = 2.0_f64 * t1983 * t22581;
    let t22584 = t1390 * t3719;
    let t22585 = t6878 * t22584;
    (t22575, t22577, t22578, t22579, t22580, t22581, t22583, t22584, t22585)
}
