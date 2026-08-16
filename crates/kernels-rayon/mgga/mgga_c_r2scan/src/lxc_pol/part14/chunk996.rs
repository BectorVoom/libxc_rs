//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 996/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk996(t2583: f64, t3308: f64, t574: f64, t2559: f64, t3295: f64, t2563: f64, t10776: f64, t2568: f64, t10772: f64, t2574: f64, t2578: f64, t10710: f64, t8128: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11797 = t3308 * t2583;
    let t11798 = t574 * t11797;
    let t11800 = t3295 * t2559;
    let t11802 = t3308 * t2563;
    let t11803 = t10776 * t11802;
    let t11805 = t3308 * t2568;
    let t11806 = t10772 * t11805;
    let t11808 = t3308 * t2574;
    let t11809 = t10776 * t11808;
    let t11811 = t3308 * t2578;
    let t11812 = t10772 * t11811;
    let t11816 = t10710 * t8128;
    (t11797, t11798, t11800, t11802, t11803, t11805, t11806, t11808, t11809, t11811, t11812, t11816)
}
