//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2013/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2013(t9108: f64, t94: f64, t102: f64, t9174: f64, t12512: f64, t580: f64, t1404: f64, t3931: f64, t1395: f64, t3946: f64, t12537: f64, t576: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35577 = t94 * t9108;
    let t35761 = t102 * t9174;
    let t39022 = t12512 * t580;
    let t39024 = t3931 * t1404;
    let t39026 = t1395 * t3946;
    let t39028 = t576 * t12537;
    (t35577, t35761, t39022, t39024, t39026, t39028)
}
