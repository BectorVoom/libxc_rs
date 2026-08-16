//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 965/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk965(t237: f64, t7511: f64, t713: f64, t722: f64, t7474: f64, t730: f64, t1957: f64, t2873: f64, t7314: f64, t7413: f64, t7415: f64, t7417: f64, t7446: f64, t7485: f64, t7491: f64, t7493: f64, t7504: f64, t7552: f64, t7554: f64, t7557: f64, t7559: f64, t7562: f64, t7564: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7566 = 0.19751673498613801407e-1_f64 * t237 * t7511;
    let t7568 = t713 * t7474 * t722;
    let t7570 = 0.5848223622634646207e0_f64 * t730 * t7568;
    let t7571 = t2873 * t1957;
    let t7573 = 0.35089341735807877242e1_f64 * t730 * t7571;
    let t7574 = t7314 - t7552 + t7554 + t7557 - t7559 - t7562 - t7564 + t7566 + t7413 + t7415 + t7417 + t7446 - t7485 + t7491 + t7493 - t7504 - t7570 - t7573;
    (t7566, t7568, t7570, t7571, t7573, t7574)
}
