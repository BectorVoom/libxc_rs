//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1293/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1293(t20305: f64, t626: f64, t20308: f64, t20343: f64, t1858: f64, t6470: f64, t1851: f64, t6483: f64, t22453: f64, t576: f64, t112: f64, t22430: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t75592 = t626 * t20305;
    let t75601 = t626 * t20308;
    let t75613 = t626 * t20343;
    let t75768 = t6470 * t1858;
    let t75774 = t1851 * t6483;
    let t75780 = t576 * t22453;
    let t75784 = t22430 * t112;
    (t75592, t75601, t75613, t75768, t75774, t75780, t75784)
}
