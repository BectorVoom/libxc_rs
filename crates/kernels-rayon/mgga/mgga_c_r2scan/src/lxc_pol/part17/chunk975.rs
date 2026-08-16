//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 975/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk975(t2574: f64, t3308: f64, t10776: f64, t2578: f64, t10772: f64, t10710: f64, t8128: f64, t10768: f64, t10781: f64, t2547: f64, t2207: f64, t3336: f64, t3606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11808 = t3308 * t2574;
    let t11809 = t10776 * t11808;
    let t11811 = t3308 * t2578;
    let t11812 = t10772 * t11811;
    let t11816 = t10710 * t8128;
    let t11817 = t10768 * t11816;
    let t11819 = t10781 * t2547;
    let t11822 = t2207 * t3336 * t3606;
    (t11808, t11809, t11811, t11812, t11816, t11817, t11819, t11822)
}
