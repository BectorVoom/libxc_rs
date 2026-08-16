//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1254/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1254(t2860: f64, t9205: f64, t2870: f64, t9242: f64, t10949: f64, t1987: f64, t10956: f64, t9352: f64, t10960: f64, t7299: f64, t730: f64, t9397: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30747 = 0.17544670867903938621e1_f64 * t2860 * t9205;
    let t30749 = 0.17544670867903938621e1_f64 * t9242 * t2870;
    let t30751 = 0.35089341735807877242e1_f64 * t1987 * t10949;
    let t30753 = 0.10254018858216406658e4_f64 * t1987 * t10956;
    let t30755 = 0.51947577317044391276e2_f64 * t2860 * t9352;
    let t30758 = 0.5848223622634646207e0_f64 * t1987 * t10960;
    let t30761 = 0.31168546390226634765e3_f64 * t730 * t9397 * t7299;
    (t30747, t30749, t30751, t30753, t30755, t30758, t30761)
}
