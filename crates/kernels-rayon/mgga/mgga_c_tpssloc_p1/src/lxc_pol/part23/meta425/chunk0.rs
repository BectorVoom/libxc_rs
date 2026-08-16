//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1254/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1254(t10403: f64, t10422: f64, t21525: f64, t18030: f64, t4630: f64, t17884: f64, t4644: f64, t13969: f64, t21502: f64, t3039: f64, t1041: f64, t21550: f64) -> (f64, f64, f64, f64, f64) {
    let t70535 = t10403 * t10422 * t21525;
    let t70554 = t18030 * t4630;
    let t70573 = t4644 * t17884;
    let t70597 = t3039 * t13969 * t21502;
    let t70640 = t1041 * t13969 * t21550;
    (t70535, t70554, t70573, t70597, t70640)
}
