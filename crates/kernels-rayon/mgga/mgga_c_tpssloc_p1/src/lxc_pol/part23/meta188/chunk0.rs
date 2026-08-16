//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 823/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk823(t10544: f64, t2840: f64, t287: f64, t275: f64, t10294: f64, t891: f64, t2843: f64, t290: f64, t10629: f64, t315: f64, t2884: f64, t307: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10636 = 0.55403703703703703703e-1_f64 * t10544;
    let t10660 = 1.0_f64 / t2840 / t287;
    let t10661 = t275 * t10660;
    let t10675 = 0.36514074074074074075e0_f64 * t10294;
    let t10676 = 0.93011851851851851854e0_f64 * t10544;
    let t10701 = 1.0_f64 / t2840 / t891;
    let t10702 = t275 * t10701;
    let t10704 = 1.0_f64 / t2843 / t290;
    let t10756 = t315 * t10629;
    let t10770 = 1.0_f64 / t2884 / t307;
    (t10636, t10660, t10661, t10675, t10676, t10701, t10702, t10704, t10756, t10770)
}
