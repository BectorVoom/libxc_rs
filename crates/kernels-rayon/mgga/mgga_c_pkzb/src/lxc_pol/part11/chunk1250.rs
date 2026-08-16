//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1250/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1250(t237: f64, t30410: f64, t30440: f64, t30466: f64, t30498: f64, t30548: f64, t30617: f64, t30663: f64, t30700: f64, t1116: f64, t25656: f64, t2860: f64, t9398: f64) -> (f64, f64, f64) {
    let t30704 = t237 * (t30410 + t30440 + t30466 + t30498 + t30548 + t30617 + t30663 + t30700);
    let t30706 = 0.17544670867903938621e1_f64 * t25656 * t1116;
    let t30708 = 0.31168546390226634765e3_f64 * t2860 * t9398;
    (t30704, t30706, t30708)
}
