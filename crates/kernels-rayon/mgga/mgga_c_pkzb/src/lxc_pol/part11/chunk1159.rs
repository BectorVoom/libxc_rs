//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1159/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1159(t154: f64, t18989: f64, t3757: f64, t385: f64, t3730: f64, t6446: f64, t28063: f64, t6524: f64, t6456: f64, t10212: f64, t10214: f64, t2380: f64, t54: f64) -> (f64, f64, f64, f64, f64) {
    let t28166 = t385 * t154 * t18989 * t3757;
    let t28174 = t385 * t154 * t6446 * t3730;
    let t28188 = t6524 * t28063;
    let t28195 = t6456 * t28063;
    let t28227 = t2380 * t54 * t10212 * t10214;
    (t28166, t28174, t28188, t28195, t28227)
}
