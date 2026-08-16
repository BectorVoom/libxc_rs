//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 638/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk638(t2860: f64, t732: f64, t1116: f64, t1987: f64, t1107: f64, t1954: f64, t723: f64, t730: f64, t2848: f64, t713: f64, t722: f64, t1976: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2862 = 0.5848223622634646207e0_f64 * t2860 * t732;
    let t2864 = 0.5848223622634646207e0_f64 * t1987 * t1116;
    let t2865 = t1954 * t1107;
    let t2866 = t2865 * t723;
    let t2868 = 0.11696447245269292414e1_f64 * t730 * t2866;
    let t2870 = t713 * t2848 * t722;
    let t2872 = 0.5848223622634646207e0_f64 * t730 * t2870;
    let t2873 = t1976 * t1107;
    (t2862, t2864, t2865, t2866, t2868, t2870, t2872, t2873)
}
