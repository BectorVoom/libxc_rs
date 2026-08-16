//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2718/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2718(t67030: f64, t75706: f64, t75733: f64, t75762: f64, t1858: f64, t6470: f64, t1851: f64, t6483: f64, t22453: f64, t576: f64, t112: f64, t22430: f64) -> (f64, f64, f64, f64, f64) {
    let t75764 = t67030 + t75706 + t75733 + t75762;
    let t75768 = t6470 * t1858;
    let t75774 = t1851 * t6483;
    let t75780 = t576 * t22453;
    let t75784 = t22430 * t112;
    (t75764, t75768, t75774, t75780, t75784)
}
